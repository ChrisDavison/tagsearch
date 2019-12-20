#[derive(Debug)]
pub struct Filter<'a> {
    good_keywords: Vec<&'a str>,
    bad_keywords: Vec<&'a str>,
    or_filter: bool,
}

impl Filter<'_> {
    pub fn new<'a>(keywords: &'a [&str], or_filter: bool) -> Filter<'a> {
        let mut good = Vec::new();
        let mut bad = Vec::new();
        for &kw in keywords {
            if kw.starts_with("!") {
                bad.push(&kw[1..]);
            } else {
                good.push(kw);
            }
        }
        Filter {
            good_keywords: good,
            bad_keywords: bad,
            or_filter,
        }
    }

    pub fn matches(&self, tags: &[String]) -> bool {
        let mut num_matches: usize = 0;
        for tag in tags {
            if self.bad_keywords.contains(&tag.as_str()) {
                return false;
            } else if self.good_keywords.contains(&tag.as_str()) {
                num_matches += 1;
            }
        }
        if self.or_filter {
            num_matches > 0
        } else {
            num_matches >= self.good_keywords.len()
        }
    }
}
