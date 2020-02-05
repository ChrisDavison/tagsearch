use structopt::StructOpt;

use tagsearch::{filter, utility::*};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tagsearch",
    about = "search for, and/or summarise, tags in plaintext files",
    version = "0.9.1"
)]
struct Opt {
    /// Keywords to filter by (prefix with ! for negative-match)
    keywords: Vec<String>,

    /// List all tags for files matching keywords
    #[structopt(short, long)]
    list: bool,

    /// Long list (e.g. tall) all tags for files matching keywords
    #[structopt(long)]
    long: bool,

    // /// Show number of files for each tag, and sort by number of files
    // #[structopt(short, long)]
    // numeric: bool,
    /// Filter using ANY, rather than ALL keywords
    #[structopt(short, long)]
    or_filter: bool,

    /// Show untagged files
    #[structopt(short, long)]
    untagged: bool,

    /// Show similar tags
    #[structopt(long)]
    similar_tags: bool,
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    let kws = args
        .keywords
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    let f = filter::Filter::new(kws.as_ref(), args.or_filter);

    let files = get_files(None)?;

    if args.untagged {
        display_untagged(f, &files);
    } else if args.similar_tags {
        display_similar_tags(f, &files);
    } else if args.list || args.long || args.keywords.is_empty() {
        display_tags(f, &files, args.long);
    } else {
        display_files_matching_query(f, &files);
    }

    Ok(())
}

fn display_untagged(f: filter::Filter, files: &[String]) {
    if let Ok(untagged) = f.untagged_files(&files) {
        for fname in untagged {
            println!("{}", fname);
        }
    }
}

fn display_similar_tags(f: filter::Filter, files: &[String]) {
    if let Ok(similar) = f.similar_tags(&files) {
        if !similar.is_empty() {
            println!("Similar tags:");
            for (issue, key1, key2) in similar {
                println!("{} - {} & {}", issue, key1, key2);
            }
        }
    }
}

fn display_files_matching_query(f: filter::Filter, files: &[String]) {
    if let Ok(matching) = f.files_matching_tag_query(&files) {
        println!("{}", matching.join("\n"));
    }
}

fn display_tags (f: filter::Filter, files: &[String], long_list: bool){
    let joinchar = if long_list { "\n" } else { ", " };
    if let Ok(tags) = f.tags_matching_tag_query(files.to_vec()) {
        println!("{}", tags.join(joinchar));
    }
}
