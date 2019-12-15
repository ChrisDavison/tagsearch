use super::*;

pub fn tags_matching_tag_query(f: filter::Filter, long_list: bool) -> Result<()> {
    let mut tagset: Set<String> = Set::new();
    for entry in super::get_files()? {
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
    let matching_files: Vec<String> = super::get_files()?
        .iter()
        .map(|fname| (fname, get_tags_for_file(&fname)))
        .filter(|(_, tags)| f.matches(tags))
        .map(|(fname, _)| fname.to_str().unwrap().to_string())
        .collect();
    println!("{}", matching_files.join("\n"));

    Ok(())
}

pub fn untagged_files() -> Result<()> {
    for entry in super::get_files()? {
        if get_tags_for_file(&entry).is_empty() {
            println!("{:?}", entry);
        }
    }
    Ok(())
}
