use structopt::StructOpt;

use tagsearch::{filter, utility::*};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tagsearch",
    about = "search for, and/or summarise, tags in plaintext files",
    version = "0.9.1"
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
}

fn main() {
    let args = Opt::from_args();
    let f = filter::Filter::new(
        args.keywords.clone().as_slice(),
        args.not.clone().as_slice(),
        args.or_filter,
    );

    let files = match get_files(None) {
        Ok(files) => files,
        Err(e) => {
            println!("Error getting files: {}", e);
            std::process::exit(1)
        }
    };

    if args.untagged {
        display_untagged(f, &files);
    } else if args.similar_tags {
        display_similar_tags(f, &files);
    } else if args.count {
        display_tag_count(f, &files);
    } else if args.list || args.long || args.keywords.is_empty() {
        display_tags(f, &files, args.long);
    } else {
        display_files_matching_query(f, &files);
    }
}

fn display_untagged(f: filter::Filter, files: &[String]) {
    for fname in f.untagged_files(files) {
        println!("{}", fname);
    }
}

fn display_similar_tags(f: filter::Filter, files: &[String]) {
    let similar = f.similar_tags(&files);
    if !similar.is_empty() {
        println!("Similar tags:");
        for issue in similar {
            println!("{}", issue);
        }
    }
}

fn display_files_matching_query(f: filter::Filter, files: &[String]) {
    println!("{}", f.files_matching_tag_query(&files).join("\n"));
}

fn display_tags(f: filter::Filter, files: &[String], long_list: bool) {
    let joinchar = if long_list { "\n" } else { ", " };
    println!(
        "{}",
        f.tags_matching_tag_query(files.to_vec()).join(joinchar)
    );
}

fn display_tag_count(f: filter::Filter, files: &[String]) {
    for (count, key) in f.count_of_tags(&files) {
        println!("{:5} {}", count, key);
    }
}
