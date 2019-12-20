use structopt::StructOpt;

use tagsearch::{filter, list, utility::Result};

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

    if args.untagged {
        list::untagged_files()?;
    } else if args.similar_tags {
        list::similar_tags()?;
    } else if args.list || args.keywords.is_empty() {
        list::tags_matching_tag_query(f, args.long)?;
    } else {
        list::files_matching_tag_query(f)?;
    }

    Ok(())
}
