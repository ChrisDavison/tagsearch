% tagsearch(1) Search and report on tags
%
% 2021-03-21


# NAME

tagsearch

# SYNOPSIS

    tagsearch [FLAGS] [OPTIONS] [\-\-] [keywords]...

Search for all **keyword** tags for files under the current directory.

# FLAGS

**-c**, **\-\-count**
: Show count of tags

**-h**, **\-\-help**
: Prints help information

**-l**, **\-\-list**
: List all tags for files matching keywords

**\-\-long**
: Long list (e.g. tall) all tags for files matching keywords

**-o**, **\-\-or-filter**
: Filter using ANY, rather than ALL keywords

**\-\-similar-tags**
: Show similar tags

**-u**, **\-\-untagged**
: Show untagged files

**-V**, **\-\-version**
: Prints version information

**-v**, **\-\-vim**
: Output format suitable for vim quickfix

**\-\-not NOT...**
: Keywords to inverse filter (i.e. ignore matching files)

# NOTES

A **tag** is something that matches the regexp **\@[a-zA-Z1-9/]+**, 
i.e. **\@** followed by anything **alphanumeric** or **/** (to allow for 'heirarchy').

    @rust
    @linux
    @c99
    @programming/c99

# AUTHORS

Chris Davison <c.jr.davison@gmail.com>
