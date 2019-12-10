package main

type tagWithCount struct {
	tag   string
	count int
}

type taglistWithCount []tagWithCount

func (t taglistWithCount) Less(i, j int) bool {
	return t[i].count < t[j].count
}

func (t taglistWithCount) Swap(i, j int) {
	t[i], t[j] = t[j], t[i]
}

func (t taglistWithCount) Len() int {
	return len(t)
}
