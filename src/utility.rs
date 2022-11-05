use std::collections::BTreeSet as Set;
use std::fs::File;
use std::io::Read;

use super::Tag;
use glob::{glob, PatternError};

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

fn is_valid_tag_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '-' || ch == '/'
}

pub fn get_tags_from_string(contents: &str) -> Set<Tag> {
    let mut keywords = Set::new();
    for line in contents.lines() {
        for word in line.split_whitespace() {
            if !word.starts_with('@') {
                continue;
            }
            let mut is_valid = true;
            for ch in word[1..].chars() {
                if !is_valid_tag_char(ch) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid && !word[1..].is_empty() {
                keywords.insert(parse_heirarchical_tag(&word[1..]));
            }
        }
    }
    keywords
}

pub fn display_as_tree(heirarchy: &[Tag]) -> String {
    let mut heirarchy: Vec<Tag> = heirarchy.to_vec();
    heirarchy.sort();
    let mut path: Vec<String> = vec![];
    let mut output = String::new();
    for tagset in heirarchy {
        // println!("NEW HEIRARCHY {:?}", tagset);
        for (i, tag) in tagset.iter().enumerate() {
            if let Some(t) = path.get(i) {
                if t == tag {
                    continue;
                } else {
                    while path.len() > i {
                        // println!("UNWINDING {:?}", path);
                        path.pop();
                    }
                    path.push(tag.to_string());
                }
            } else {
                path.push(tag.to_string());
            }
            let indents = "    ".repeat(path.len() - 1);
            output.push_str(&format!("{}{}\n", indents, path[path.len() - 1]));
        }
    }
    output
}

fn parse_heirarchical_tag(s: &str) -> Vec<String> {
    s.trim_start_matches('@')
        .split('/')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::collections::BTreeSet as Set;

    #[test]
    fn test_tags_from_string() {
        let output = vec![vec!["a"], vec!["b"], vec!["c"], vec!["d", "e", "f"]]
            .iter()
            .cloned()
            .map(|v| v.iter().map(|x| x.to_string()).collect())
            .collect::<Set<Vec<String>>>();
        let input = "@a @b @c @d/e/f";
        assert_eq!(get_tags_from_string(input), output);
    }

    #[test]
    fn display_as_tree_test() {
        let output2 = String::from("completely\n    unrelated\n        heirarchy\nphilosophy\n    mindset\n    stoicism\n        quote\n");
        let input2 = vec![
            vec![
                "philosophy".to_string(),
                "stoicism".to_string(),
                "quote".to_string(),
            ],
            vec!["philosophy".to_string(), "mindset".to_string()],
            vec![
                "completely".to_string(),
                "unrelated".to_string(),
                "heirarchy".to_string(),
            ],
        ];
        assert_eq!(display_as_tree(&input2), output2);
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
