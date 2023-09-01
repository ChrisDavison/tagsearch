use crate::utility::get_tags_for_file;

use std::collections::{BTreeMap as Map, BTreeSet as Set};

use super::Tag;
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
#[derive(Debug, Default)]
pub struct Filter<'a> {
    good_keywords: Set<&'a str>,
    bad_keywords: Set<&'a str>,
    or_filter: bool,
}

// TODO change issue to contain Tag instead of String
#[derive(Eq, PartialEq,Debug)]
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

impl<'a> Filter<'a> {
    /// Create a new `Filter`
    ///
    /// This simply takes the good and bad keywords and turns them into a
    /// vector. It also sets whether the filter is AND or OR-based.
    pub fn new<S: AsRef<str>>(
        keywords: &'a [S],
        bad_keywords: &'a [S],
        or_filter: bool,
    ) -> Filter<'a> {
        Filter {
            good_keywords: keywords.iter().map(|x| x.as_ref()).collect(),
            bad_keywords: bad_keywords.iter().map(|x| x.as_ref()).collect(),
            or_filter,
        }
    }

    /// Check if a set of tags matches the filter
    ///
    /// This takes a bunch of tags that have been pulled from a file, and
    /// checks if the good and bad keywords match.
    ///
    /// TODO probably want to use a set here to make sure I'm matching all terms
    /// rather than just asuming that if I have more matches than good keywords that it's good.
    /// ...if so, how do I handle heirarchical tags?
    /// ...only look for the sub-words of the heirarchy, or count it as both?
    pub fn matches(&self, tags: &Set<Tag>) -> bool {
        let mut num_matching_tags: usize = 0;
        // Here 'tags' is what we've pulled from the file
        for heirarchicaltag in tags {
            for tag in heirarchicaltag {
                let tag_l = tag.to_lowercase();
                if self.tag_matches(&self.bad_keywords, &tag_l) {
                    return false;
                }
                if self.tag_matches(&self.good_keywords, &tag_l) {
                    num_matching_tags += 1;
                }
            }
            if self.tag_matches(
                &self.good_keywords,
                &heirarchicaltag.join("/").to_lowercase(),
            ) {
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
    fn tag_matches(&self, v: &Set<&str>, t: &str) -> bool {
        v.iter()
            .any(|haystack| t.contains(&haystack.to_lowercase()))
    }

    /// Extract ALL tags from files that match a filter
    ///
    /// Given a set of filenames (as `String`s), check if each file matches
    /// the filter. If a file matches, gather all its tags.
    pub fn tags_matching_tag_query(&self, files: &[String]) -> Set<Tag> {
        files
            .par_iter()
            .map(|x| get_tags_for_file(x))
            .filter(|x| {
                if !(self.good_keywords.is_empty() || self.bad_keywords.is_empty()) {
                    self.matches(x)
                } else {
                    true
                }
            })
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
        let mut tagset: Set<Tag> = Set::new();
        files.iter().for_each(|entry| {
            tagset.extend(get_tags_for_file(entry));
        });
        let mut similar = Vec::new();
        for ts1 in &tagset {
            for ts2 in &tagset {
                if let Some(issue) = Filter::compare_heirarchical_tags(ts1, ts2) {
                    if !similar.contains(&issue) {
                        similar.push(issue);
                    }
                }
            }
        }
        similar
    }

    fn compare_heirarchical_tags(t1: &Tag, t2: &Tag) -> Option<Issue> {
        // Compare each component of the heirarchy
        // rather than treating it as a single string
        for (key, key2) in t1.iter().zip(t2.iter()) {
            if key == key2 {
                continue;
            }
            if key.to_lowercase() == key2.to_lowercase() {
                // Ensure we don't add B-A if we've flagged A-B
                return Some(Issue::Case(key.to_string(), key2.to_string()));
            } else if key.trim_end_matches('s') == key2.trim_end_matches('s') {
                // Ensure we don't add B-A if we've flagged A-B
                return Some(Issue::Plural(key.to_string(), key2.to_string()));
            }
        }
        None
    }

    /// Count the number of occurences of each tag
    ///
    /// This will count how many files each tag appears in. The returned
    /// vector is sorted high to low.
    pub fn count_of_tags(&self, files: &[String]) -> Vec<(usize, String)> {
        let mut tagmap: Map<String, usize> = Map::new();
        for entry in files {
            for tag in get_tags_for_file(entry) {
                for subtag in &tag {
                    match tagmap.get_mut(subtag) {
                        Some(val) => *val += 1,
                        None => {
                            tagmap.insert(subtag.to_string(), 1);
                        }
                    }
                }

                if tag.len() > 1 {
                    let fulltag = tag.join("/");
                    match tagmap.get_mut(&fulltag) {
                        Some(val) => *val += 1,
                        None => {
                            tagmap.insert(fulltag, 1);
                        }
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
    use crate::utility::parse_heirarchical_tag as tagparse;

    /// This macro just streamlines the repetitive filter creation and set creation.
    macro_rules! tag_match {
        (good [$($good:literal),*] bad [$($bad:literal),*] file_tags [$($filetags:literal),+] $case:literal $negate:literal) => {
            let f = Filter::new(&[$($good),*], &[$($bad),*], $case);

            // Make every passed 'filetag' a vec![ft.to_string()]
            // e.g. vec![ vec![ft1.to_string()], vec![ft2.to_string()] ...]
            let tags_for_fake_file = [$(vec![$filetags.to_string()]),+]
                .iter()
                .cloned()
                .collect::<Set<Vec<String>>>();

            let matches = f.matches(&tags_for_fake_file);
            let passes = if $negate { !matches } else {matches};
            assert!(passes);
        };
        ([$($input:literal),+] - [$($bad:literal),*] or_matches [$($output:literal),+]) => {
            tag_match!(good [$($input),*] bad [$($bad),*] file_tags [$($output),+] true false);
        };
        ([$($input:literal),*] - [$($bad:literal),*] matches [$($output:literal),+]) => {
            tag_match!(good [$($input),*] bad [$($bad),*] file_tags [$($output),+] false false);
        };
        ([$($bad:literal),+] rejects [$($output:literal),+]) => {
            tag_match!(good [] bad [$($bad),+] file_tags [$($output),+] true true);
        };
    }

    macro_rules! tag_compare {
        (plural $first:literal is like $second:literal) => {
            assert_eq!(
                Filter::compare_heirarchical_tags(&tagparse($first), &tagparse($second)), 
                Some(Issue::Plural($first.to_string(), $second.to_string())));
        };
        (lowercase $first:literal is like lowercase $second:literal) => {
            assert_eq!(
                Filter::compare_heirarchical_tags(&tagparse($first), &tagparse($second)), 
                Some(Issue::Case($first.to_string(), $second.to_string())));
        };
        ($first:literal is not like $second:literal) => {
            assert_eq!(
                Filter::compare_heirarchical_tags(&tagparse($first), &tagparse($second)), 
                None)
        }
    }

    #[test]
    fn match_good() {
        tag_match!(["stoicism", "philosophy"] - [] matches ["stoicism", "philosophy"]);
        tag_match!(["stoicism", "PHILOSOPHY"] - [] matches ["STOICISM", "pHiLoSOpHy"]);
        tag_match!(["stoicism", "philosophy"] - [] or_matches ["stoicism", "philosophy"]);
        tag_match!(["stoicism", "philosophy"] - [] or_matches ["stoicism"]);
        tag_match!(["stoicism", "philosophy"] - [] or_matches ["philosophy"]);
        tag_match!(["stoicism", "philosophy"] - ["donkey"] or_matches ["philosophy"]);
    }

    #[test]
    fn match_bad() {
        tag_match!(["donkey"] rejects ["stoicism", "philosophy", "donkey"]);
    }

    #[test]
    fn compare_tags(){
        tag_compare!(plural "as" is like "a");
        tag_compare!(plural "a/b/cs" is like "a/b/c");
        tag_compare!(plural "a/bs/c" is like "a/b/c");
        tag_compare!(lowercase "A" is like lowercase "a");
        tag_compare!(lowercase "A/b/c" is like lowercase "a/b/c");
        tag_compare!(lowercase "a/B/c" is like lowercase "a/b/c");
        tag_compare!("As" is not like "a");
    }
}
