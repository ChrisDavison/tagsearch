use crate::filter;
use crate::utility::{get_files, get_tags_for_file, Result};

use std::collections::BTreeSet as Set;

pub fn tags_matching_tag_query(f: filter::Filter, long_list: bool) -> Result<()> {
    let mut tagset: Set<String> = Set::new();
    for entry in get_files()? {
        let tags = get_tags_for_file(&entry);
        if f.matches(&tags) {
            tagset.extend(tags);
        }
    }
    let tagkeys = tagset.iter().cloned().collect::<Vec<String>>();
    let joinchar = if long_list { "\n" } else { ", " };
    println!("{}", tagkeys.join(joinchar));

    Ok(())
}

pub fn files_matching_tag_query(f: filter::Filter) -> Result<()> {
    let matching_files: Vec<String> = get_files()?
        .iter()
        .map(|fname| (fname, get_tags_for_file(&fname)))
        .filter(|(_, tags)| f.matches(tags))
        .map(|(fname, _)| fname.to_str().unwrap().to_string())
        .collect();
    println!("{}", matching_files.join("\n"));

    Ok(())
}

pub fn untagged_files() -> Result<()> {
    for entry in get_files()? {
        if get_tags_for_file(&entry).is_empty() {
            println!("{:?}", entry);
        }
    }
    Ok(())
}

pub fn similar_tags() -> Result<()> {
    let mut tagset: Set<String> = Set::new();
    for entry in get_files()? {
        let tags = get_tags_for_file(&entry);
        tagset.extend(tags);
    }
    let tagkeys = tagset.iter().cloned().collect::<Vec<String>>();
    let mut similar = Vec::new();
    for key in &tagkeys {
        for key2 in &tagkeys {
            if key == key2 {
                continue;
            } else if key.to_lowercase() == key2.to_lowercase() {
                similar.push(("CASE", key, key2));
            } else if key.trim_end_matches('s') == key2.trim_end_matches('s') {
                similar.push(("PLURAL", key, key2));
            }
        }
    }
    if !similar.is_empty() {
        println!("Similar tags:");
        for (issue, key1, key2) in similar {
            println!("{} - {} & {}", issue, key1, key2);
        }
    }
    Ok(())
}
