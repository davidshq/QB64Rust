# $EMBED

`$EMBED:'file','handle'` builds a file into the executable, for `_EMBEDDED$("handle")` to read.

## Syntax

```text
$EMBED:'file','handle'
```

## Behavior

The file is the text between the line's first two `'`, the handle between the next two, blanks at their ends dropped;
the rest of the line is dropped. Any letter case of `$EMBED` is accepted.
<!-- src: language/embedded-files "The $EMBED line"; tests\corpus\slice\s138_embed -->

A name starting with `./` or `.\` is in the main source file's folder, also when the line stands in an included file.
An absolute name is taken as written; any other relative name is looked for from the compiler's folder.
<!-- src: language/embedded-files "Where the file is"; tests\corpus\slice\s138_embed; tests\reference\examples\meta-embed_1 -->

A handle is a letter followed by letters and digits, and letter case counts: `h` and `H` are two handles.
<!-- src: language/embedded-files "Handles"; tests\corpus\slice\s138_embed -->

A file is built in only when a compiled `_EMBEDDED$` names its handle.
<!-- src: language/embedded-files "Only used files are built in"; tests\corpus\slice\s138_embed -->

In a comment it is a comment, and in a `$IF` branch not taken it names nothing.
<!-- src: language/embedded-files "The $EMBED line" -->

## Errors

- A name that is not a file, and a file or handle named by two `$EMBED` lines, are compile errors. The second names
  the first one's line.
  <!-- src: language/embedded-files "Where the file is"; language/embedded-files "Handles" -->
- A name with a drive and no slash after it is reported "not supported yet".
  <!-- src: language/embedded-files "Where the file is" -->

## Differences

From QB64pe: for a file that is not there or is a folder, its message names the folder it looked in. QB64Rust's
gives the name as the program writes it: "the file `./name` of `$EMBED` is not found" (`DIVERGENCES-QB64PE.md`
D-015).
<!-- src: D-015 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-embed_1 -->
```qb64
$EMBED:'./greeting.txt','greeting'
g$ = _EMBEDDED$("greeting")
PRINT LEN(g$); "bytes: "; g$
END
```

Output:

```text
 17 bytes: Hello from a file

Press any key to continue
```
<!-- end generated -->

## See also

`_EMBEDDED$`, `$INCLUDE`.
