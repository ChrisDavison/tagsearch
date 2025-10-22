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
        /// Match ANY, not ALL, tags
        #[arg(short, long)]
        or: bool,
        /// Query to process [good -bad filename]
        query: Option<Vec<String>>,
    },
    /// Show all tags from files with tags that match filter words
    #[command(aliases=&["t"])]
    Tags {
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
        /// Query to process
        query: Option<Vec<String>>,
    },
    /// Show files without tags
    #[command(aliases=&["u"])]
    Untagged,
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
        Commands::Files { or, query } => {
            let (_files, good, not) = if let Some(query) = query {
                parse_positionals(&query)
            } else {
                (vec![], vec![], vec![])
            };
            let files = if _files.is_empty() {
                files
            } else {
                _files.iter().map(|x| x.to_string()).collect()
            };
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            display_files_matching_query(f, &files)
        }
        Commands::Tags {
            or,
            count,
            long,
            per_file,
            query,
        } => {
            let (_files, good, not) = if let Some(query) = query {
                parse_positionals(&query)
            } else {
                (vec![], vec![], vec![])
            };
            dbg!(&_files);
            dbg!(&good);
            dbg!(&not);
            let files = if _files.is_empty() {
                files
            } else {
                _files.iter().map(|x| x.to_string()).collect()
            };
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            if count {
                display_tag_count(f, &files, per_file)
            } else {
                display_tags(f, &files, long, per_file)
            }
        }
        Commands::Untagged => display_untagged(&files),
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

fn display_untagged(files: &[String]) -> Result<(), std::io::Error> {
    let untagged: String = files
        .par_iter()
        .filter(|x| get_tags_for_file(x).is_empty())
        .cloned()
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

fn display_files_matching_query(f: Filter, files: &[String]) -> Result<(), std::io::Error> {
    writeln!(
        &mut std::io::stdout(),
        "{}",
        f.files_matching_tag_query(files).join("\n")
    )
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
