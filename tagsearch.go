package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"

	"gopkg.in/alecthomas/kingpin.v2"
)

const USAGE = `usage: tagsearch [-h|-v|-l] <FILES>... (-f <keywords>...)

Search/list tags in files. Tags are taken as '@' followed by anything in the 
character class [a-zA-Z0-9_\-].

Options:
	-l                 List tags
	-v                 List tags verbosely (tag and number of files containing tag)
	-h                 Show this help message
	-f <keywords>...   Keywords to find
`

var (
	list      = kingpin.Flag("list", "List tags").Short('l').Bool()
	verbose   = kingpin.Flag("verbose", "List tags with count of files").Short('v').Bool()
	summarise = kingpin.Flag("summarise", "List tags and matching files").Short('s').Bool()
	files     = kingpin.Arg("files", "Files to search/summarise").Strings()
	keywords  = kingpin.Flag("keywords", "Keywords to filter for").Short('k').Strings()
	andFilter = kingpin.Flag("and-filter", "Filter using ALL tags (or default ANY match)").Bool()
)

func getAllTags(files []string) taglistWithCount {
	tagsSeen := make(map[string]int)
	for _, filename := range files {
		for _, tag := range getTagsForFile(filename) {
			tagsSeen[tag] += 1
		}
	}
	taglist := make(taglistWithCount, 0)
	for tag, count := range tagsSeen {
		taglist = append(taglist, tagWithCount{tag, count})
	}
	sort.Sort(sort.Reverse(taglist))
	return taglist
}

func listTags(files []string, verbose bool) {
	if verbose {
		for _, countedTag := range getAllTags(files) {
			fmt.Printf("%5d %s\n", countedTag.count, countedTag.tag)
		}

	} else {
		tagnames := make([]string, 0)
		for _, countedTag := range getAllTags(files) {
			tagnames = append(tagnames, countedTag.tag)
		}
		fmt.Println(strings.Join(tagnames, ", "))
	}

}

func getTagsForFile(filename string) []string {
	contents, err := ioutil.ReadFile(filename)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return []string{}
	}
	rx := regexp.MustCompile(`(?:^|\s)+@([a-zA-Z_0-9\-]+)\s*`)
	matches := rx.FindAllStringSubmatch(string(contents), -1)
	matchesSeen := make(map[string]bool)
	for _, match := range matches {
		for _, keyword := range match[1:] {
			matchesSeen[keyword] = true
		}
	}
	onlyGroups := make([]string, 0)
	for kw, _ := range matchesSeen {
		onlyGroups = append(onlyGroups, strings.ToLower(kw))
	}
	return onlyGroups
}

func keywordMapOr(keywordToFile map[string][]string, keywords []string) []string {
	results := make([]string, 0)
	filenamesSeen := make(map[string]bool)
	for _, kw := range keywords {
		for _, filename := range keywordToFile[kw] {
			filenamesSeen[filename] = true
		}
	}
	for filename, _ := range filenamesSeen {
		results = append(results, filename)
	}
	return results
}

func keywordMapAnd(keywordToFile map[string][]string, keywords []string) []string {
	results := make([]string, 0)
	countOfKeywords := make(map[string]int)
	nilMatches := make([]string, 0)
	for _, kw := range keywords {
		files, ok := keywordToFile[kw]
		if !ok {
			fmt.Printf("No matches for `%s`\n", kw)
			nilMatches = append(nilMatches, kw)
			continue
		}
		for _, file := range files {
			countOfKeywords[file] += 1
		}
	}
	if nilMatches == nil {
		fmt.Println("No matches for: ", strings.Join(nilMatches, ", "))
	}
	for filename, count := range countOfKeywords {
		if count == len(keywords) {
			results = append(results, filename)
		}
	}
	return results
}

func main() {
	kingpin.Parse()
	var err error
	if *files == nil {
		*files, err = filepath.Glob("**/*.txt")
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		filesMd, err := filepath.Glob("**/*.md")
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
		}
		*files = append(*files, filesMd...)
	}
	if *verbose || *list || (*keywords == nil && !*summarise) {
		listTags(*files, *verbose)
		os.Exit(0)
	}

	keywordToFile := make(map[string][]string, 0)
	for _, filename := range *files {
		for _, tag := range getTagsForFile(filename) {
			keywordToFile[tag] = append(keywordToFile[tag], filename)
		}
	}
	if *summarise {
		for kw, files := range keywordToFile {
			fmt.Println(kw)
			for _, filename := range files {
				fmt.Println("\t", filename)
			}
		}
		os.Exit(0)
	}
	matchingFiles := make([]string, 0)
	if *andFilter {
		matchingFiles = keywordMapAnd(keywordToFile, *keywords)
	} else {
		matchingFiles = keywordMapOr(keywordToFile, *keywords)
	}
	for _, filename := range matchingFiles {
		fmt.Println(filename)
	}
}
