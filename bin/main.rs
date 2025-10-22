use std::io::Write;

use rayon::prelude::*;
use tagsearch::{filter::Filter, utility::*};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    root: Option<String>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Show files that have tags matching filter words
    #[command(aliases=&["f"])]
    Files {
        /// Keywords to match
        #[arg(long)]
        good: Vec<String>,
        /// Keywords to NOT match
        #[arg(long)]
        not: Vec<String>,
        /// Output in format suitable for vimgrep
        vim: bool,
        #[arg(long)]
        /// Match ANY, not ALL, tags
        #[arg(short, long)]
        or: bool,
        /// Files to process
        files: Option<Vec<String>>,
    },
    /// Show all tags from files with tags that match filter words
    #[command(aliases=&["t"])]
    Tags {
        /// Keywords to match
        #[arg(long)]
        good: Vec<String>,
        /// Keywords to NOT match
        #[arg(long)]
        not: Vec<String>,
        /// Match ANY, not ALL, tags
        #[arg(short, long)]
        or: bool,
        /// Show how many times tag used
        #[arg(short, long)]
        count: bool,
        /// Output vertically
        #[arg(short, long)]
        long: bool,
        /// Tags per-file
        #[arg(long)]
        per_file: bool,
        /// Files to process
        files: Option<Vec<String>>,
    },
    /// Show files without tags
    #[command(aliases=&["u"])]
    Untagged {
        /// Output in format suitable for vimgrep
        #[arg(long)]
        vim: bool,
    },
    /// Show tags that may be typos/slight differences
    #[command(aliases=&["similar", "related", "s"])]
    SimilarTags,
}

fn try_main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    let files = match get_files(cli.root) {
        Ok(files) => files,
        Err(e) => {
            println!("Error getting files: {}", e);
            std::process::exit(1)
        }
    };

    match cli.command {
        Commands::Files {
            good,
            not,
            vim,
            or,
            files: files2,
        } => {
            let files = files2.unwrap_or(files);
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            display_files_matching_query(f, &files, vim)
        }
        Commands::Tags {
            good,
            not,
            or,
            count,
            long,
            per_file,
            files: files2,
        } => {
            let files = files2.unwrap_or(files);
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            if count {
                display_tag_count(f, &files, per_file)
            } else {
                display_tags(f, &files, long, per_file)
            }
        }
        Commands::Untagged { vim } => display_untagged(&files, vim),
        Commands::SimilarTags => display_similar_tags(&files),
    }
}

fn main() {
    if let Err(e) = try_main() {
        if e.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn display_untagged(files: &[String], vim_format: bool) -> Result<(), std::io::Error> {
    let untagged: String = files
        .par_iter()
        .filter(|x| get_tags_for_file(x).is_empty())
        .map(|x| {
            if vim_format {
                format!("{}:1:NO TAGS", x)
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    writeln!(&mut std::io::stdout(), "{}:1:NO TAGS", untagged)?;
    Ok(())
}

fn display_similar_tags(files: &[String]) -> Result<(), std::io::Error> {
    let f = Filter::default();
    let similar = f.similar_tags(files);
    if !similar.is_empty() {
        writeln!(&mut std::io::stdout(), "Similar tags:")?;
        for issue in similar {
            writeln!(&mut std::io::stdout(), "{}", issue)?;
        }
    }
    Ok(())
}

fn display_files_matching_query(
    f: Filter,
    files: &[String],
    vim_format: bool,
) -> Result<(), std::io::Error> {
    if vim_format {
        let mut vimstrings: Vec<String> = Vec::new();
        for filename in f.files_matching_tag_query(files) {
            let contents = std::fs::read_to_string(filename.clone())?;
            for (i, line) in contents.lines().enumerate() {
                let tags_in_line = get_tags_from_string(line);
                if tags_in_line.is_empty() {
                    continue;
                }
                if f.matches(&tags_in_line) {
                    vimstrings.push(format!("{}:{}:1:{}", filename, i + 1, line));
                }
            }
        }
        writeln!(&mut std::io::stdout(), "{}", vimstrings.join("\n"))?;
    } else {
        writeln!(
            &mut std::io::stdout(),
            "{}",
            f.files_matching_tag_query(files).join("\n")
        )?;
    }
    Ok(())
}

fn display_tags(
    f: Filter,
    files: &[String],
    long_list: bool,
    per_file: bool,
) -> Result<(), std::io::Error> {
    // Convert the Btreeset into a vec

    if per_file {
        for fname in files {
            let tags: Vec<String> = f
                .tags_matching_tag_query(&[fname.to_string()])
                .iter()
                .map(|tags| tags.join("/"))
                .collect();
            if tags.is_empty() {
                continue;
            }
            if long_list {
                writeln!(&mut std::io::stdout(), "{}", fname)?;
                for t in tags {
                    writeln!(&mut std::io::stdout(), "- {}", t)?;
                }
                writeln!(&mut std::io::stdout())?;
            } else {
                writeln!(&mut std::io::stdout(), "{}: {}", fname, tags.join(", "))?;
            };
        }
    } else {
        let tags: Vec<String> = f
            .tags_matching_tag_query(files)
            .iter()
            .map(|tags| tags.join("/"))
            .collect();
        let tagstr = if long_list {
            tags.join("\n")
        } else {
            tags.join(", ")
        };

        writeln!(&mut std::io::stdout(), "{}", tagstr)?;
    }
    Ok(())
}

fn display_tag_count(f: Filter, files: &[String], per_file: bool) -> Result<(), std::io::Error> {
    if per_file {
        for fname in files {
            for (count, key) in f.count_of_tags(&[fname.to_string()]) {
                writeln!(&mut std::io::stdout(), "{:5} {}", count, key)?;
            }
        }
    } else {
        for (count, key) in f.count_of_tags(files) {
            writeln!(&mut std::io::stdout(), "{:5} {}", count, key)?;
        }
    }
    Ok(())
}
