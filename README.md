# Tagsearch - Search for, and/or summarise, tags in plaintext files.

![Rust](https://github.com/chrisdavison/tagsearch/workflows/Rust/badge.svg)

## Introduction

This finds all instances of `@ALPHANUMERIC` (e.g. roughly matches the regex
`@[a-zA-Z0-9]`) across `.txt`, `.md`, and `.org` recursively under the current 
directory.

The *purpose* of this script is to basically let me manage information in
plaintext much more easily. I've added a separate file giving [a rough overview
of how I apply it to different usecases][].

  [a rough overview of how I apply it to different usecases]: ./use_cases.md

## Motivation

A prime motivation was to move away from more commercial software. Even though I
can afford to use a few subscription services, I feel like it's better to
simplify my processes down. This has multiple benefits:

-   saving money; not subscribing to as many services
-   no vendor lock-in; don't need to worry about losing my data if a vendor goes
    out of business, or changes their pricing model
-   less 'failure modes'; with a simple plaintext system, built the way I want
    it to, I don't have to worry about all the interactions or working the way
    that other software requires me to. Everything is basically human readable,
    with a few helper scripts to handle the data in different ways.
-   portability; I don't need to worry about finding an app that works across
    Windows, OSX, and Linux. I can realistically access my data across any
    device that can handle text files. Every device I currently work with has
    the ability to search within files, so even without tools such as this, I
    can still filter my information down.

The main bit of commercial software I still use is *Dropbox*, so I can get
information across all my devices without having to worry about doing things
like syncing git. I do however have a cronjob which backs up this information to
git. One change this makes to my process is using the file extension `.txt`
instead of `.md` for my markdown files; this simply lets me utilise Dropbox's
full-text search from the web or android app to find information when I don't
have access to a terminal and my own helper scripts.

## Installation

Can be either installed from source, or installed with `cargo`:

***From source:***

```
git clone git@github.com:Chrisdavison/tagsearch
cd tagsearch
cargo install --path . --force
```

***From cargo***

`cargo install tagsearch`

### Dependencies

    regex = "1"
    glob = "0.3.0"
    lazy_static = "1.4.0"
    structopt = "0.3.3"

## Usage

Current usage string:

```
tagsearch 0.27.0
search for, and/or summarise, tags in plaintext files

USAGE:
    tagsearch [FLAGS] [OPTIONS] [--] [keywords]...

FLAGS:
    -c, --count           Show count of tags
    -h, --help            Prints help information
    -l, --list            List all tags for files matching keywords
        --long            Long list (e.g. tall) all tags for files matching keywords
    -o, --or-filter       Filter using ANY, rather than ALL keywords
        --similar-tags    Show similar tags
    -u, --untagged        Show untagged files
    -V, --version         Prints version information
    -v, --vim             Output format suitable for vim quickfix

OPTIONS:
        --not <not>...    Keywords to inverse filter (i.e. ignore matching files)

ARGS:
    <keywords>...    Keywords to filter
```

Examples

```
# display all tags
tagsearch                

# display all tags in a column (like ls -l)
tagsearch --long          

# Show files tagged 'golang'
tagsearch golang          

# List all tags that occur along with golang
tagsearch -l golang       

# Show files tagged 'rust', but NOT tagged 'video'
tagsearch rust --not video   

# Show files that match spanish OR espanol
tagsearch -o spanish espanol   
```
