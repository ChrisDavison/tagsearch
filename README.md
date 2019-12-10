# Tagsearch

Search for, and/or summarise, tags (`@keyword`) in plaintext files.

This is a utility which should help drive my move to managing most things in
plaintext.

## Installation

Download the repo

    go get github.com/chrisdavison/tagsearch

Change to the repo directory

    cd $GOPATH/src/github.com/ChrisDavison/tagsearch

Download dependencies

    go get

Install

    go install

### Dependencies

-   [kingpin][] (for CLI flag parsing)

  [kingpin]: https://github.com/alecthomas/kingpin

## Usage

Current usage string:

    usage: tagsearch [<flags>] [<keyword>...]

    Flags:
        --help       Show context-sensitive help (also try --help-long and --help-man).
    -l, --list       List tags
        --long       Long (tall) list of tags
    -n, --numeric    When listing, sort by number of tags, and show number
    -s, --summarise  List tags and matching files
        --or-filter  Filter using ANY tags, rather than ALL
        --version    Show version

    Args:
    [<keyword>]  Keywords to filter (prepend '!' to ignore keyword)

Examples

    tagsearch                 # defaults to -l|--long
    tagsearch --long          # to show a tall list of tags
    tagsearch --long -n       # Tall list, with count of files for each tag
    tagsearch --summarise     # Show each tag, and the files for each tag

## Use-cases

### Academic Literature

I have pdfs of academic literature saved. Now, I use a fish shell helper
function that: 1) downloads a pdf, 2) renames the pdf to whatever I input, in a
folder `papers`, 3) creates a txt file with the same name as the pdf, in folder
`writeups`, with the tag `@unread`. I can optionally ad more tags.

This then lets me use `tagsearch unread neuralnet` to find any paper related to
neural nets that I've not read yet, or `tagsearch -l neuralnet` to see all tags
that also exist with the `neuralnet` tag.

### Finance

I keep a folder called `budget`, that contains files of the form:

    title: WHAT I BOUGHT
    value: A NUMBER
    date: WHEN I BOUGHT IT

    @monthly @health

I can then use tagsearch to see what I often spend monthly on
`tagsearch -l @monthly`, or do some pipelining to calculate some sums (using
fish-shell syntax):

    awk -F':' '/^value:/{total+=$2} END{print $total}' < (cat (tagsearch @monthly))

...this approach is not quite as fleshed out as the academic literature

### Quantified-Self

Similarly to the [finance][] approach, but `value` is now one such as `50` (for
my `20191210--shoulderpress.txt` weight lifting entry, number of reps). I can
then either do `cat (tagsearch weights shoulder)` to print out all the contents
of any of my shoulderpress progress.

...this approach is not quite as fleshed out as the academic literature

  [finance]: #finance
