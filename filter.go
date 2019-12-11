package main

const (
	FILTER_AND = iota
	FILTER_OR
)

type filter struct {
	keywordMapGood map[string]bool
	keywordMapBad  map[string]bool
	filterMode     int
}

func NewFilter(keywords []string, orMode bool) filter {
	goodWords := make(map[string]bool)
	badWords := make(map[string]bool)
	for _, word := range keywords {
		if word[0] == '!' {
			badWords[word[1:]] = true
		} else {
			goodWords[word] = true
		}
	}
	filterMode := FILTER_OR
	if !orMode {
		filterMode = FILTER_AND
	}
	return filter{goodWords, badWords, filterMode}
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
			matchingKeywords += 1
		}
	}
	if numKeywords == 0 {
		return true
	}
	if b.filterMode == FILTER_OR && matchingKeywords > 0 {
		// if an 'or' filter, and we have any positive match, return 'true'
		return true
	} else if b.filterMode == FILTER_AND && matchingKeywords >= numKeywords {
		// if an 'and' filter, and we have the same number of keywords as required, 'true'
		return true
	}
	// else, we don't have a match
	return false
}
