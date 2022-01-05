use std::io::Write;

use structopt::StructOpt;

use rayon::prelude::*;
use tagsearch::{filter, utility::*};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tagsearch",
    about = "search for, and/or summarise, tags in plaintext files"
)]
struct Opt {
    /// Keywords to filter
    keywords: Vec<String>,

    /// Keywords to inverse filter (i.e. ignore matching files)
    #[structopt(long)]
    not: Vec<String>,

    /// List all tags for files matching keywords
    #[structopt(short, long)]
    list: bool,

    /// Long list (e.g. tall) all tags for files matching keywords
    #[structopt(long)]
    long: bool,

    /// Filter using ANY, rather than ALL keywords
    #[structopt(short, long)]
    or_filter: bool,

    /// Show untagged files
    #[structopt(short, long)]
    untagged: bool,

    /// Show count of tags
    #[structopt(short, long)]
    count: bool,

    /// Show similar tags
    #[structopt(long)]
    similar_tags: bool,

    /// Fuzzy-match tags
    #[structopt(short, long)]
    fuzzy: bool,

    /// Output format suitable for vim quickfix
    #[structopt(short, long)]
    vim: bool,
}

fn main() {
    let args = Opt::from_args();
    let f = filter::Filter::new(
        args.keywords.as_slice(),
        args.not.as_slice(),
        args.or_filter,
        args.fuzzy,
    );

    let files = match get_files(None) {
        Ok(files) => files,
        Err(e) => {
            println!("Error getting files: {}", e);
            std::process::exit(1)
        }
    };

    if let Err(e) = if args.untagged {
        display_untagged(f, &files, args.vim)
    } else if args.similar_tags {
        display_similar_tags(f, &files)
    } else if args.count {
        display_tag_count(f, &files)
    } else if args.list || args.long || args.keywords.is_empty() {
        display_tags(f, &files, args.long)
    } else {
        display_files_matching_query(f, &files, args.vim)
    } {
        if e.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn display_untagged(
    f: filter::Filter,
    files: &[String],
    vim_format: bool,
) -> Result<(), std::io::Error> {
    for fname in f.untagged_files(files) {
        if vim_format {
            writeln!(&mut std::io::stdout(), "{}:1:Ignore this message", fname)?;
        } else {
            writeln!(&mut std::io::stdout(), "{}", fname)?;
        }
    }
    Ok(())
}

fn display_similar_tags(f: filter::Filter, files: &[String]) -> Result<(), std::io::Error> {
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
    f: filter::Filter,
    files: &[String],
    vim_format: bool,
) -> Result<(), std::io::Error> {
    if vim_format {
        writeln!(
            &mut std::io::stdout(),
            "{}",
            f.files_matching_tag_query(files)
                .par_iter()
                .map(|fname| format!("{}:1:", fname))
                .collect::<Vec<String>>()
                .join("\n")
        )?;
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
    f: filter::Filter,
    files: &[String],
    long_list: bool,
) -> Result<(), std::io::Error> {
    let joinchar = if long_list { "\n" } else { ", " };
    writeln!(
        &mut std::io::stdout(),
        "{}",
        f.tags_matching_tag_query(files)
            .iter()
            .cloned()
            .map(|tagset| tagset.join("/"))
            .collect::<Vec<String>>()
            .join(joinchar)
    )?;
    Ok(())
}

fn display_tag_count(f: filter::Filter, files: &[String]) -> Result<(), std::io::Error> {
    for (count, key) in f.count_of_tags(files) {
        writeln!(&mut std::io::stdout(), "{:5} {}", count, key)?;
    }
    Ok(())
}
