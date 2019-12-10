package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"regexp"
	"sort"
	"strings"

	"github.com/bmatcuk/doublestar"
	"gopkg.in/alecthomas/kingpin.v2"
)

const VERSION = "0.4.0"

var (
	list        = kingpin.Flag("list", "List tags").Short('l').Bool()
	longList    = kingpin.Flag("long", "Long (tall) list of tags").Bool()
	numericSort = kingpin.Flag("numeric", "When listing, sort by number of tags, and show number").Short('n').Bool()
	summarise   = kingpin.Flag("summarise", "List tags and matching files").Short('s').Bool()
	orFilter    = kingpin.Flag("or-filter", "Filter using ANY tags, rather than ALL").Bool()
	version     = kingpin.Flag("version", "Show version").Bool()
	keywords    = kingpin.Arg("keyword", "Keywords to filter (prepend '!' to ignore keyword)").Strings()
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

type countedKeyword struct {
	kw    string
	files []string
}

type countedKeywords []countedKeyword

func listTags(filesForKeyword map[string][]string, summarise, longList, numericSort bool) {
	countedKeywordList := make(countedKeywords, 0)
	for kw, files := range filesForKeyword {
		countedKeywordList = append(countedKeywordList, countedKeyword{kw, files})
	}
	if numericSort {
		sort.SliceStable(countedKeywordList, func(i, j int) bool {
			return len(countedKeywordList[i].files) > len(countedKeywordList[j].files)
		})
	} else {
		sort.SliceStable(countedKeywordList, func(i, j int) bool {
			return countedKeywordList[i].kw < countedKeywordList[j].kw
		})
	}

	tagnamesAndCountsStrings := make([]string, 0)
	for _, ck := range countedKeywordList {
		kwTitle := ck.kw
		if numericSort {
			kwTitle += fmt.Sprintf(" - %d", len(ck.files))
		}
		if summarise {
			fmt.Println(kwTitle)
			for _, filename := range ck.files {
				fmt.Println("\t", filename)
			}
		} else {
			tagnamesAndCountsStrings = append(tagnamesAndCountsStrings, kwTitle)
		}
	}
	if !summarise {
		inter := ", "
		if longList {
			inter = "\n"
		}
		fmt.Println(strings.Join(tagnamesAndCountsStrings, inter))
	}
}

func getTagsForFile(filename string) []string {
	contents, err := ioutil.ReadFile(filename)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		return []string{}
	}
	rx := regexp.MustCompile(`(?:^|\s)@([a-zA-Z_0-9\-]+)`)
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

type fileAndTags struct {
	filename string
	tags     []string
}

func getFileList() ([]string, error) {
	files, err := doublestar.Glob("**/*txt")
	if err != nil {
		return nil, err
	}
	filesMd, err := doublestar.Glob("**/*md")
	if err != nil {
		return nil, err
	}
	files = append(files, filesMd...)
	return files, nil
}

func main() {
	kingpin.Parse()
	if *version {
		fmt.Printf("tagsearch v%s\n", VERSION)
		os.Exit(0)
	}
	var err error

	files, err := getFileList()
	if err != nil {
		log.Fatal(err)
	}

	filter := NewBitstringFilter(*keywords, *orFilter)
	matchingTaggedFiles := make([]fileAndTags, 0)
	for _, fn := range files {
		tags := getTagsForFile(fn)
		if filter.Matches(tags) {
			matchingTaggedFiles = append(matchingTaggedFiles, fileAndTags{fn, tags})
		}
	}

	keywordToFile := make(map[string][]string)

	for _, taggedFile := range matchingTaggedFiles {
		for _, tag := range taggedFile.tags {
			keywordToFile[tag] = append(keywordToFile[tag], taggedFile.filename)
		}
	}

	if *list || *longList || *summarise || *keywords == nil {
		listTags(keywordToFile, *summarise, *longList, *numericSort)
	} else {
		for _, taggedFile := range matchingTaggedFiles {
			fmt.Println(taggedFile.filename)
		}
	}
}
