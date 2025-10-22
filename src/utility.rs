use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;

use super::Tag;
use glob::{glob, PatternError};

const HEIRARCHY_SPLITTERS: [char; 2] = [':', '/'];

/// Get all files from either a passed path or under the current directory.
///
/// This will do a recursive glob for `.txt` and `.md` files. If the `root`
/// argument is `None`, then the current directory will be used; otherwise,
/// the given path will be used.
pub fn get_files(root: Option<String>) -> Result<Vec<String>, PatternError> {
    let dir = root.unwrap_or(".".to_string());
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
    get_tags_from_string(&contents)
}

fn is_valid_tag_char(ch: char) -> bool {
    ch.is_alphanumeric() || "-/:_".contains(ch)
}

fn only_numeric(s: &str) -> bool {
    s.chars().all(|x| x.is_numeric())
}

pub fn get_tags_from_string(contents: &str) -> Set<Tag> {
    let mut keywords = Set::new();
    for line in contents.lines() {
        for word in line.split_whitespace() {
            let word = match word.strip_prefix("\u{feff}") {
                Some(w) => w.to_string(),
                None => word.to_string(),
            };
            if !(word.starts_with('@') || word.starts_with('#')) {
                continue;
            }
            let mut is_valid = true;
            for ch in word[1..].chars() {
                if !is_valid_tag_char(ch) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid && !word[1..].is_empty() && !only_numeric(&word[1..]) {
                keywords.insert(parse_heirarchical_tag(&word[1..]));
            }
        }
    }
    keywords
}

pub fn parse_heirarchical_tag(s: &str) -> Vec<String> {
    s.trim_start_matches('@')
        .trim_start_matches('#')
        .split(|c: char| HEIRARCHY_SPLITTERS.contains(&c))
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn parse_positionals(s: &[String]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut files = Vec::new();
    let mut good = Vec::new();
    let mut bad = Vec::new();
    for w in s {
        match w.chars().next().unwrap() {
            '-' | '!' => bad.push(w[1..].to_string()),
            '+' => good.push(w[1..].to_string()),
            _ => {
                if std::fs::exists(w).unwrap_or(false) {
                    if std::path::PathBuf::from(w).is_dir() {
                        files
                            .extend_from_slice(&get_files(Some(w.to_string())).unwrap_or_default());
                    } else {
                        files.push(w.to_string());
                    }
                }
            }
        }
    }
    (files, good, bad)
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::collections::BTreeSet as Set;

    #[allow(dead_code, unused_macros)]
    macro_rules! svec {
        ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    };
    }

    #[allow(dead_code, unused_macros)]
    macro_rules! set {
        ( $( $x:expr ),* ) => {
        {
            let mut temp_set = Set::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
    }

    #[test]
    fn test_tags_from_string() {
        let input = "@a @b @c @d/e/f";
        assert_eq!(
            get_tags_from_string(input),
            vec![
                vec!["a".to_string()],
                vec!["b".to_string()],
                vec!["c".to_string()],
                vec!["d".to_string(), "e".to_string(), "f".to_string()],
            ]
            .iter()
            .cloned()
            .collect::<Set<_>>()
        );
    }

    #[test]
    fn test_parse_heirarchical_tag() {
        assert_eq!(parse_heirarchical_tag("@d/e/f"), vec!["d", "e", "f"]);
        assert_eq!(parse_heirarchical_tag("@single"), vec!["single"]);
        assert_eq!(
            parse_heirarchical_tag("@delta/gamma"),
            vec!["delta", "gamma"]
        );
    }

    #[test]
    fn test_parse_positionals() {
        assert_eq!(
            parse_positionals(&[
                "+good".to_string(),
                "-bad".to_string(),
                "README.md".to_string()
            ]),
            (
                vec!["README.md".to_string()],
                vec!["good".to_string()],
                vec!["bad".to_string()]
            )
        );
    }
}
