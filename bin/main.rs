use std::io::Write;

use tagsearch::{filter::Filter, utility::*, Tag};

use structopt::StructOpt;

#[derive(StructOpt,Debug)]
struct Cli {
    #[structopt(subcommand)]
    command: Commands,
    #[structopt(long)]
    root: Option<String>,
}

#[derive(StructOpt,Debug)]
enum Commands {
    /// Show files that have tags matching filter words
    #[structopt(aliases=&["f"])]
    Files {
        /// Keywords to match
        good: Vec<String>,
        #[structopt(long, require_delimiter(true))]
        /// Keywords to NOT match
        not: Vec<String>,
        /// Output in format suitable for vimgrep
        #[structopt(long)]
        vim: bool,
        /// Match ANY, not ALL, tags
        #[structopt(short, long)]
        or: bool,
    },
    /// Show all tags from files with tags that match filter words
    #[structopt(aliases=&["t"])]
    Tags {
        /// Keywords to match
        good: Vec<String>,
        #[structopt(long, require_delimiter(true))]
        /// Keywords to NOT match
        not: Vec<String>,
        #[structopt(short, long)]
        /// Match ANY, not ALL, tags
        or: bool,
        /// Show how many times tag used
        #[structopt(short, long)]
        count: bool,
        /// Output in long format (tree-like)
        #[structopt(short, long)]
        long: bool,
        /// Stop 'tree' output in long list
        #[structopt(short, long)]
        no_tree: bool,
    },
    /// Show tags from specific files
    #[structopt(aliases=&["ft"])]
    FileTags {
        /// Show how many times tag used
        #[structopt(short, long)]
        count: bool,
        /// Output in long format (tree-like)
        #[structopt(short, long)]
        long: bool,
        /// Stop 'tree' output in long list
        #[structopt(short, long)]
        no_tree: bool,
        /// Files to extract tags from
        files: Vec<String>,
    },
    /// Show files without tags
    #[structopt(aliases=&["u"])]
    Untagged {
        /// Output in format suitable for vimgrep
        #[structopt(long)]
        vim: bool,
    },
    /// Show tags that may be typos/slight differences
    #[structopt(aliases=&["similar", "related", "s"])]
    SimilarTags,
}

fn try_main() -> Result<(), std::io::Error> {
    let cli = Cli::from_args();
    let files = match get_files(cli.root) {
        Ok(files) => files,
        Err(e) => {
            println!("Error getting files: {}", e);
            std::process::exit(1)
        }
    };

    match cli.command {
        Commands::Files { good, not, vim, or } => {
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            display_files_matching_query(f, &files, vim)
        }
        Commands::Tags {
            good,
            not,
            or,
            count,
            long,
            no_tree,
        } => {
            let f = Filter::new(good.as_slice(), not.as_slice(), or);
            if count {
                display_tag_count(f, &files)
            } else {
                display_tags(f, &files, long, no_tree)
            }
        }
        Commands::FileTags {
            count,
            long,
            no_tree,
            files,
        } => {
            let f: Filter = Default::default();
            if count {
                display_tag_count(f, &files)
            } else {
                display_tags(f, &files, long, no_tree)
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
    let f = Filter::default();
    for fname in f.untagged_files(files) {
        if vim_format {
            writeln!(&mut std::io::stdout(), "{}:1:NO TAGS", fname)?;
        } else {
            writeln!(&mut std::io::stdout(), "{}", fname)?;
        }
    }
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
    no_tree: bool,
) -> Result<(), std::io::Error> {
    // Convert the Btreeset into a vec
    let tags: Vec<Tag> = f.tags_matching_tag_query(files).iter().cloned().collect();

    if long_list {
        if !no_tree {
            writeln!(&mut std::io::stdout(), "{}", display_as_tree(&tags))?;
        } else {
            let tags = tags
                .iter()
                .map(|tags| tags.join("/"))
                .collect::<Vec<String>>()
                .join("\n");
            writeln!(&mut std::io::stdout(), "{}", tags)?;
        }
    } else {
        let tags = tags
            .iter()
            .map(|tags| tags.join("/"))
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(&mut std::io::stdout(), "{}", tags)?;
    }
    Ok(())
}

fn display_tag_count(f: Filter, files: &[String]) -> Result<(), std::io::Error> {
    for (count, key) in f.count_of_tags(files) {
        writeln!(&mut std::io::stdout(), "{:5} {}", count, key)?;
    }
    Ok(())
}
