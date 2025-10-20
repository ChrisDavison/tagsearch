package main

import (
	"bufio"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync"
)

type TagSet map[string]bool
type TagPair struct {
	filename string
	tags     []string
}

func filesWithSuffix(root, suffix string) []string {
	var hits []string
	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, e error) error {
		if e != nil { // permission problems, etc.
			return nil // skip and continue
		}
		if d.IsDir() { // keep recursing
			return nil
		}
		if strings.HasSuffix(path, suffix) {
			hits = append(hits, path)
		}
		return nil
	})

	if err != nil {
		fmt.Fprintln(os.Stderr, "walk error:", err)
		os.Exit(1)
	}

	return hits
}

func fileTags(filename string, ch chan TagPair) {
	// Compile the regex: [#@]{1}[a-zA-Z/]
	// This matches a single # or @ followed by a letter or slash
	re := regexp.MustCompile(`[#@]{1}[a-zA-Z/]{3,}`)

	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	fbuf := bufio.NewScanner(f)
	file_tags := make(TagSet)

	for fbuf.Scan() {
		line := fbuf.Text()

		// Find all matches
		matches := re.FindAllString(line, -1)

		for _, m := range matches {
			file_tags[m] = true
		}
	}
	var file_strings []string
	for tag := range file_tags {
		file_strings = append(file_strings, tag)
	}
	fss := sort.StringSlice(file_strings)
	fss.Sort()

	ch <- TagPair{filename, fss}
}

func tagsForFiles(files []string) (map[string][]string, []string) {
	file_tags := make(map[string][]string)
	file_tags_strings := make(map[string][]string)

	all_tag_set := make(map[string]bool)

	channel := make(chan TagPair, len(files))
	var wg sync.WaitGroup

	for _, filename := range files {
		wg.Go(func() {
			fileTags(filename, channel)
		})
	}
	for i := 0; i < len(files); i++ {
		ft := <-channel
		if len(ft.tags) == 0 {
			continue
		}
		log.Println(ft)
		for _, t := range ft.tags {
			all_tag_set[t] = true
		}
		file_tags[ft.filename] = ft.tags
	}

	var all_tags []string
	for tag, _ := range all_tag_set {
		all_tags = append(all_tags, tag)
	}
	sorted := sort.StringSlice(all_tags)
	sorted.Sort()

	return file_tags_strings, all_tags
}

func main() {
	var files []string
	command := "tags"
	if len(os.Args) == 1 {
		files = filesWithSuffix(".", ".md")
	} else {
		switch os.Args[1] {
		case "tags":
			command = "tags"
		case "file-tags":
			command = "filetags"
		case "filetags":
			command = "filetags"
		default:
			fmt.Fprintln(os.Stderr, "usage: tagsearch [tags|filetags] FILES...")
			os.Exit(1)
		}
		if len(os.Args[2:]) > 0 {
			for _, arg := range os.Args[2:] {
				if s, _ := os.Stat(arg); s.IsDir() {
					files = append(files, filesWithSuffix(arg, ".md")...)
				} else {
					files = append(files, arg)
				}
			}
		} else {
			files = filesWithSuffix(".", ".md")
		}
	}

	file_tags, all_tags := tagsForFiles(files)

	switch command {
	case "tags":
		fmt.Println(strings.Join(all_tags, "\n"))
	case "filetags":
		for ft, tags := range file_tags {
			fmt.Printf("%s: %s\n", ft, strings.Join(tags, ", "))
		}
	}
}
