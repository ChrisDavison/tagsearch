use crate::utility::{get_tags_for_file, Result};

use std::collections::{BTreeSet as Set,BTreeMap as Map};

#[derive(Debug)]
pub struct Filter {
    good_keywords: Vec<String>,
    bad_keywords: Vec<String>,
    or_filter: bool,
}

impl Filter {
    pub fn new(keywords: &[String], bad_keywords: &[String], or_filter: bool) -> Filter {
        Filter {
            good_keywords: keywords.to_vec(),
            bad_keywords: bad_keywords.to_vec(),
            or_filter,
        }
    }

    pub fn matches(&self, tags: &[String]) -> bool {
        let mut num_matches: usize = 0;
        for tag in tags {
            if self.bad_keywords.contains(tag) {
                return false;
            } else if self.good_keywords.contains(tag) {
                num_matches += 1;
            }
        }
        if self.or_filter {
            num_matches > 0
        } else {
            num_matches >= self.good_keywords.len()
        }
    }
    pub fn tags_matching_tag_query(&self, files: Vec<String>) -> Result<Vec<String>> {
        let mut tagset: Set<String> = Set::new();
        for entry in files {
            let tags = get_tags_for_file(&entry);
            if self.matches(&tags) {
                tagset.extend(tags);
            }
        }

        Ok(tagset.iter().cloned().collect::<Vec<String>>())
    }

    pub fn files_matching_tag_query(&self, files: &[String]) -> Result<Vec<String>> {
        let matching_files: Vec<String> = files
            .iter()
            .filter(|fname| self.matches(get_tags_for_file(&fname).as_ref()))
            .map(|fname| fname.to_string())
            .collect();

        Ok(matching_files)
    }

    pub fn untagged_files(&self, files: &[String]) -> Result<Vec<String>> {
        Ok(files
            .iter()
            .filter(|x| get_tags_for_file(&x).is_empty())
            .map(|x| x.to_string())
            .collect())
    }

    pub fn similar_tags(&self, files: &[String]) -> Result<Vec<(String, String, String)>> {
        let mut tagset: Set<String> = Set::new();
        for entry in files {
            let tags = get_tags_for_file(&entry);
            tagset.extend(tags);
        }
        let mut similar = Vec::new();
        for key in &tagset {
            for key2 in &tagset {
                let mut issue = String::new();
                if key == key2 {
                    continue;
                } else if key.to_lowercase() == key2.to_lowercase() {
                    // Ensure we don't add B-A if we've flagged A-B
                    issue = String::from("CASE");
                } else if key.trim_end_matches('s') == key2.trim_end_matches('s') {
                    // Ensure we don't add B-A if we've flagged A-B
                    issue = String::from("PLURAL")
                }
                if !issue.is_empty() {
                    let elem = (issue.clone(), key.to_string(), key2.to_string());
                    if !similar.contains(&(issue, key2.to_string(), key.to_string())) {
                        similar.push(elem);
                    }
                }
            }
        }
        Ok(similar)
    }

    pub fn count_of_tags(&self, files: &[String]) -> Result<Vec<(usize, String)>> {
        let mut tagmap: Map<String, usize> = Map::new();
        for entry in files {
            for tag in  get_tags_for_file(&entry){
                match tagmap.get_mut(&tag) {
                    Some(val) => *val += 1,
                    None => {tagmap.insert(tag, 1); ()},
                }
            }
        }
        let mut out = Vec::new();
        for (key, val) in tagmap {
            out.push((val, key));
        }
        out.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        Ok(out)
    }
}
