use lazy_static;
use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;

use glob::{glob, PatternError};
use regex::Regex;

/// Get all files from either a passed path or under the current directory.
///
/// This will do a recursive glob for `.txt` and `.md` files. If the `root`
/// argument is `None`, then the current directory will be used; otherwise,
/// the given path will be used.
pub fn get_files(root: Option<String>) -> Result<Vec<String>, PatternError> {
    let dir = match root {
        Some(d) => d,
        None => ".".to_string(),
    };
    let mut files = Vec::new();
    let txts = glob(&format!("{}/**/*.txt", dir))?;
    let mds = glob(&format!("{}/**/*.md", dir))?;
    let orgs = glob(&format!("{}/**/*.org", dir))?;
    for filename in txts.chain(mds).chain(orgs) {
        if let Ok(fname) = filename {
            files.push(fname.to_string_lossy().into());
        }
    }
    Ok(files)
}

/// Get all tags for a single file
///
/// This will take all 'keywords' that match from a file, where a keyword
/// is defined as `@[a-zA-Z0-9_\-]`, i.e. any alphanumeric character, `_`,
/// or `-`. The keyword must be separate from it's surroundings (e.g. `\b`
/// in regex terminology)...spaces, start or end of line, punctuation all
/// count as being a 'boundary'. The leading `@` will be stripped.
pub fn get_tags_for_file(filename: &str) -> Vec<String> {
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
