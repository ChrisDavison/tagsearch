use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

use glob::glob;
use regex::Regex;
use structopt::StructOpt;

mod filter;
mod list;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

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
    } else if args.list || args.keywords.is_empty() {
        list::tags_matching_tag_query(f, args.long)?;
    } else {
        list::files_matching_tag_query(f)?;
    }

    Ok(())
}

fn get_files() -> Result<Vec<PathBuf>> {
    Ok(glob("**/*.txt")?
        .chain(glob("**/*.md")?)
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect())
}

fn get_tags_for_file(filename: &PathBuf) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^|\s)@(?P<keyword>[a-zA-Z_0-9\-]+)")
            .expect("Couldn't create keyword regex");
    }
    let mut file = File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read contents of file");
    let mut keywords = Set::new();
    for cap in RE.captures_iter(&contents) {
        keywords.insert(cap["keyword"].to_string());
    }
    keywords.iter().cloned().collect()
}
