use lazy_static;
use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use glob::glob;
use regex::Regex;

pub type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

pub fn get_files(root: Option<String>) -> Result<Vec<PathBuf>> {
    let dir = match root {
        Some(d) => d,
        None => ".".to_string(),
    };
    Ok(glob(&format!("{}/**/*.txt", dir))?
        .chain(glob(&format!("{}/**/*.md", dir))?)
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect())
}

pub fn get_tags_for_file(filename: &PathBuf) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^|\s)@(?P<keyword>[a-zA-Z_0-9\-]+)")
            .expect("Couldn't create keyword regex");
    }
    let mut file = File::open(filename).expect(&format!("Couldn't open file: `{:?}`", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Couldn't read contents of file: `{:?}`", filename));
    let mut keywords = Set::new();
    for cap in RE.captures_iter(&contents) {
        keywords.insert(cap["keyword"].to_string());
    }
    keywords.iter().cloned().collect()
}
