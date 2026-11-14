# Embedded files

`$EMBED` builds a file into the executable under a handle, and `_EMBEDDED$` gives its bytes while the program runs, so
the file need not be shipped beside it.

The pages of this area: [$EMBED](../entries/meta-embed.md), [_EMBEDDED$](../entries/_embedded-str.md).

## From the line to the bytes

A `$EMBED` line names the file and the handle in single quotes. A name starting with `./` is in the main source file's
folder.
<!-- src: language/embedded-files "The $EMBED line"; language/embedded-files "Where the file is"; tests\reference\examples\meta-embed_1 -->

`_EMBEDDED$("handle")` takes one string literal, a handle named by a `$EMBED` line above it, and gives a new string of
the file's bytes at each call.
<!-- src: language/embedded-files "_EMBEDDED$"; tests\reference\examples\_embedded-str_1 -->

A file is built in only when a compiled `_EMBEDDED$` names its handle, so a `$EMBED` line alone costs nothing.
<!-- src: language/embedded-files "Only used files are built in"; tests\corpus\slice\s138_embed -->

## Handles

A handle is a letter followed by letters and digits, and letter case counts. A file or a handle named by two lines is
a compile error that names the first line.
<!-- src: language/embedded-files "Handles" -->
