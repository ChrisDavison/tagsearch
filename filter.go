package main

// Enum to handle the file mode
const (
	filterAnd = iota
	filterOr
)

type filter struct {
	keywordMapGood map[string]bool
	keywordMapBad  map[string]bool
	filterUsingOr  bool
}

func newFilter(keywords []string, orMode bool) filter {
	goodWords := make(map[string]bool)
	badWords := make(map[string]bool)
	for _, word := range keywords {
		if word[0] == '!' {
			badWords[word[1:]] = true
		} else {
			goodWords[word] = true
		}
	}
	return filter{goodWords, badWords, orMode}
}

func (b filter) Matches(keywordsForFile []string) bool {
	numKeywords := len(b.keywordMapGood)
	matchingKeywords := 0
	for _, keyword := range keywordsForFile {
		// fmt.Println(keyword)
		// if keyword is one of any bad word, immediately 'false' -> bad match
		if _, ok := b.keywordMapBad[keyword]; ok {
			return false
		}
		// if keyword is one of the 'good' words, increment goodword match
		if _, ok := b.keywordMapGood[keyword]; ok {
			matchingKeywords++
		}
	}
	if numKeywords == 0 {
		return true
	}
	if b.filterUsingOr && matchingKeywords > 0 {
		// if an 'or' filter, and we have any positive match, return 'true'
		return true
	} else if !b.filterUsingOr && matchingKeywords >= numKeywords {
		// if an 'and' filter, and we have the same number of keywords as required, 'true'
		return true
	}
	// else, we don't have a match
	return false
}
