# Use-cases

## Academic Literature

I have pdfs of academic literature saved. Now, I use a fish shell helper
function that: 1) downloads a pdf, 2) renames the pdf to whatever I input, in a
folder `papers`, 3) creates a txt file with the same name as the pdf, in folder
`writeups`, with the tag `@unread`. I can optionally ad more tags.

This then lets me use `tagsearch unread neuralnet` to find any paper related to
neural nets that I've not read yet, or `tagsearch -l neuralnet` to see all tags
that also exist with the `neuralnet` tag.

## Finance

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

## Quantified-Self

Similarly to the [finance][] approach, but `value` is now one such as `50` (for
my `20191210--shoulderpress.txt` weight lifting entry, number of reps). I can
then either do `cat (tagsearch weights shoulder)` to print out all the contents
of any of my shoulderpress progress.

...this approach is not quite as fleshed out as the academic literature

  [finance]: #finance
