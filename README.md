# Tagsearch

Search for, and/or summarise, tags (`@keyword`) in plaintext files.

This is a utility which should help drive my move to managing most things in plaintext.

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

- [kingpin](https://github.com/alecthomas/kingpin) (for CLI flag parsing)


## Usage

List all the tags within a directory:

    tagsearch
    # OR
    tagsearch -l (to be specifically verbose)

List all tags, with a count of how many files contain them

    tagsearch -s|--summarise

List all tags, along with the name of each associated file

    tagsearch -v|--verbose


## TODO Use-cases

### Academic Literature

### Finance

### Quantified-Self
