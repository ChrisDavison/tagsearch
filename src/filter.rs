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
    good_keywords: Set<String>,
    bad_keywords: Set<String>,
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
    pub fn matches(&self, tags: &Set<String>) -> bool {
        let mut num_matching_tags: usize = 0;
        for tag in tags {
            if self.tag_matches(&self.bad_keywords, tag) {
                return false;
            }
            if self.tag_matches(&self.good_keywords, tag) {
                num_matching_tags += 1;
            }
        }
        let matches_required = if self.or_filter {
            1 // If we are using an or filter, any 1 match is good enough
        } else {
            self.good_keywords.len() // Otherwise, we must match all keywords
        };
        num_matching_tags >= matches_required
    }

    #[inline(always)]
    fn tag_matches(&self, v: &Set<String>, t: &str) -> bool {
        if self.fuzzy_match {
            v.iter().any(|haystack| t.contains(&haystack.to_string()))
        } else {
            v.contains(&t.to_string())
        }
    }

    /// Extract ALL tags from files that match a filter
    ///
    /// Given a set of filenames (as `String`s), check if each file matches
    /// the filter. If a file matches, gather all its tags.
    pub fn tags_matching_tag_query(&self, files: &[String]) -> Set<String> {
        files
            .par_iter()
            .map(|x| get_tags_for_file(x))
            .filter(|x| self.matches(x))
            .flatten()
            .collect()
    }

    /// Extract all files that match a filter
    ///
    /// Given a set of filenames (as `String`s), filter to only those containing matching tags.
    pub fn files_matching_tag_query(&self, files: &[String]) -> Vec<String> {
        files
            .par_iter()
            .filter(|fname| self.matches(&get_tags_for_file(fname)))
            .map(|fname| fname.to_string())
            .collect::<Vec<String>>()
    }

    /// Get all files without tags
    pub fn untagged_files(&self, files: &[String]) -> Vec<String> {
        files
            .par_iter()
            .filter(|x| get_tags_for_file(x).is_empty())
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
            let tags = get_tags_for_file(entry);
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
            for tag in get_tags_for_file(entry) {
                match tagmap.get_mut(&tag) {
                    Some(val) => *val += 1,
                    None => {
                        tagmap.insert(tag, 1);
                    }
                }
            }
        }
        let mut out: Vec<_> = tagmap.iter().map(|(k, v)| (*v, k.clone())).collect();
        out.sort_by(|a, b| a.0.cmp(&b.0).reverse());
        out
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_good() {
        let f = Filter::new(&["stoicism", "philosophy"], &[], false, false);
        let tags_for_fake_file = [String::from("stoicism"), String::from("philosophy")]
            .iter()
            .cloned()
            .collect::<Set<String>>();

        assert!(f.matches(&tags_for_fake_file));
    }

    #[test]
    fn match_good_or() {
        let f = Filter::new(&["stoicism", "philosophy"], &[], true, false);
        let tags_for_fake_file = [String::from("stoicism"), String::from("philosophy")]
            .iter()
            .cloned()
            .collect::<Set<String>>();

        assert!(f.matches(&tags_for_fake_file));
    }

    #[test]
    fn match_good_fuzzy() {
        let f = Filter::new(&["stoic"], &[], false, true);
        let tags_for_fake_file = [String::from("stoicism")]
            .iter()
            .cloned()
            .collect::<Set<String>>();

        assert!(f.matches(&tags_for_fake_file));
    }

    #[test]
    fn match_bad() {
        let f = Filter::new(&[], &["donkey"], false, false);
        let tags_for_fake_file = [
            String::from("stoicism"),
            String::from("philosophy"),
            String::from("donkey"),
        ]
        .iter()
        .cloned()
        .collect::<Set<String>>();
        assert!(!f.matches(&tags_for_fake_file));
    }

    #[test]
    fn match_good_and_bad_fuzzy() {
        let f = Filter::new(&["stoic"], &["donkey"], false, true);
        let tags_for_fake_file = [
            String::from("stoicism"),
            String::from("philosophy"),
            String::from("donkey"),
        ]
        .iter()
        .cloned()
        .collect();
        assert!(!f.matches(&tags_for_fake_file));
    }
}
