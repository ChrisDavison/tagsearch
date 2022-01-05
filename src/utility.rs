#![allow(dead_code)]
use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;

use glob::{glob, PatternError};
use regex::Regex;
use super::Tag;

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
    for filename in txts.chain(mds).chain(orgs).flatten() {
        files.push(filename.to_string_lossy().into());
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
pub fn get_tags_for_file(filename: &str) -> Set<Tag> {
    let mut file =
        File::open(filename).unwrap_or_else(|_| panic!("Couldn't open file: `{:?}`", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Couldn't read contents of file: `{:?}`", filename));
    get_tags_from_string(&contents.clone())
}

fn get_tags_from_string(contents: &str) -> Set<Tag> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^|\s)@(?P<keyword>[a-zA-Z_0-9\-/]+)")
            .expect("Couldn't create keyword regex");
    }
    let mut keywords = Set::new();
    for cap in RE.captures_iter(contents) {
        keywords.insert(parse_heirarchical_tag(&cap["keyword"]));
    }
    keywords
}

fn parse_heirarchical_tag(s: &str) -> Vec<String> {
    s.trim_start_matches("@").split("/").map(|x| x.to_string()).collect::<Vec<String>>()
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::collections::BTreeSet as Set;

    #[test]
    fn test_tags_from_string() {
        let output = vec!["a", "b", "c"]
            .iter()
            .cloned()
            .map(|x| x.to_string())
            .collect::<Set<String>>();
        let input = "@a @b @c";
        assert_eq!(get_tags_from_string(input), output);
    }

    #[test]
    fn test_parse_heirarchical_tag() {
        let tests = vec![
            ("@d/e/f", vec!["d", "e", "f"]),
            ("@delta/gamma", vec!["delta", "gamma"]),
            ("@single", vec!["single"]),
        ];
        for (inp, outp) in tests {
            assert_eq!(parse_heirarchical_tag(inp), outp);
        }
    }
}
