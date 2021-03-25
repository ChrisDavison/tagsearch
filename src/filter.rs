use crate::utility::get_tags_for_file;

use std::collections::{BTreeMap as Map, BTreeSet as Set};

use rayon::prelude::*;

/// The `Filter` struct is used for filtering files for tags
///
/// The filter is split into 'good words' and 'bad words', i.e. tags that a
/// file MUST have and tags that a file MUST NOT have.
///
/// The filter, by default, is an AND filter, i.e. all good words must exist
/// and no bad words must exist. The filter can be made into an OR filter,
/// where a file will be returned if ANY good word matches the file and NO
/// bad words match.
#[derive(Debug)]
pub struct Filter {
    good_keywords: Vec<String>,
    bad_keywords: Vec<String>,
    or_filter: bool,
    fuzzy_match: bool,
}

#[derive(Eq, PartialEq)]
pub enum Issue {
    Plural(String, String),
    Case(String, String),
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Issue::Plural(a, b) => write!(f, "Plural - {} & {}", a, b),
            Issue::Case(a, b) => write!(f, "Case - {} & {}", a, b),
        }
    }
}

impl Filter {
    /// Create a new `Filter`
    ///
    /// This simply takes the good and bad keywords and turns them into a
    /// vector. It also sets whether the filter is AND or OR-based.
    pub fn new<S: AsRef<str>>(
        keywords: &[S],
        bad_keywords: &[S],
        or_filter: bool,
        fuzzy_match: bool,
    ) -> Filter {
        Filter {
            good_keywords: keywords.iter().map(|x| x.as_ref().to_string()).collect(),
            bad_keywords: bad_keywords
                .iter()
                .map(|x| x.as_ref().to_string())
                .collect(),
            or_filter,
            fuzzy_match,
        }
    }

    /// Check if a set of tags matches the filter
    ///
    /// This takes a bunch of tags that have been pulled from a file, and
    /// checks if the good and bad keywords match.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// let f = tagsearch::filter::Filter::new(&["work", "project1"], &["project2"], false);
    /// if f.matches(&["work", "project3", "project3"]) {
    ///     println!("MATCHES");
    /// }
    /// ```
    pub fn matches(&self, tags: &[String]) -> bool {
        let mut num_matches: usize = 0;
        let matcher = if self.fuzzy_match {
            Filter::vec_has_tag_fuzzy
        } else {
            Filter::vec_has_tag
        };
        for tag in tags {
            if matcher(&self.bad_keywords, &tag) {
                return false;
            } else if matcher(&self.good_keywords, &tag) {
                num_matches += 1;
            }
        }
        if self.or_filter {
            num_matches > 0
        } else {
            num_matches >= self.good_keywords.len()
        }
    }

    fn vec_has_tag(v: &[String], t: &str) -> bool {
        v.contains(&t.to_string())
    }

    fn vec_has_tag_fuzzy(v: &[String], t: &str) -> bool {
        v.iter().any(|haystack| t.contains(&haystack.to_string()))
    }

    /// Extract ALL tags from files that match a filter
    ///
    /// Given a set of filenames (as `String`s), check if each file matches
    /// the filter. If a file matches, append all of it's tags to a vector.
    pub fn tags_matching_tag_query(&self, files: Vec<String>) -> Vec<String> {
        let mut tagset: Set<String> = Set::new();
        for entry in files {
            let tags = get_tags_for_file(&entry);
            if self.matches(tags.as_slice()) {
                tagset.extend(tags);
            }
        }

        tagset.par_iter().cloned().collect::<Vec<String>>()
    }

    /// Extract all files that match a filter
    ///
    /// Given a set of filenames (as `String`s), check if each file matches
    /// the filter. If a file matches, append it to the vector.
    pub fn files_matching_tag_query(&self, files: &[String]) -> Vec<String> {
        let matching_files: Vec<String> = files
            .par_iter()
            .filter(|fname| self.matches(get_tags_for_file(&fname).as_ref()))
            .map(|fname| fname.to_string())
            .collect();

        matching_files
    }

    /// Get all files without tags
    pub fn untagged_files(&self, files: &[String]) -> Vec<String> {
        files
            .par_iter()
            .filter(|x| get_tags_for_file(&x).is_empty())
            .map(|x| x.to_string())
            .collect()
    }

    /// List possibly similar tags, based on some simple heuristics.
    ///
    /// This only does a simple test for case (i.e. upper vs lowercase) and
    /// plurality. The plurality check only does a simple test if one tag ends
    /// with 's' and the other doesn't.
    ///
    /// If the pair (A,B) is listed as having a problem, the pair (B,A) WILL
    /// NOT be added to the result.
    pub fn similar_tags(&self, files: &[String]) -> Vec<Issue> {
        let mut tagset: Set<String> = Set::new();
        for entry in files {
            let tags = get_tags_for_file(&entry);
            tagset.extend(tags);
        }
        let mut similar = Vec::new();
        for key in &tagset {
            for key2 in &tagset {
                if key == key2 {
                    continue;
                }
                let issue = if key.to_lowercase() == key2.to_lowercase() {
                    // Ensure we don't add B-A if we've flagged A-B
                    Issue::Case(key.to_string(), key2.to_string())
                } else if key.trim_end_matches('s') == key2.trim_end_matches('s') {
                    // Ensure we don't add B-A if we've flagged A-B
                    Issue::Plural(key.to_string(), key2.to_string())
                } else {
                    continue;
                };
                if !similar.contains(&issue) {
                    similar.push(issue);
                }
            }
        }
        similar
    }

    /// Count the number of occurences of each tag
    ///
    /// This will count how many files each tag appears in. The returned
    /// vector is sorted high to low.
    pub fn count_of_tags(&self, files: &[String]) -> Vec<(usize, String)> {
        let mut tagmap: Map<String, usize> = Map::new();
        for entry in files {
            for tag in get_tags_for_file(&entry) {
                match tagmap.get_mut(&tag) {
                    Some(val) => *val += 1,
                    None => {
                        tagmap.insert(tag, 1);
                    }
                }
            }
        }
        let mut out = Vec::new();
        for (key, val) in tagmap {
            out.push((val, key));
        }
        out.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        out
    }
}
