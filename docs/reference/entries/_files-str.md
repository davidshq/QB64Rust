# _FILES$

`_FILES$(spec$)` starts a listing of a folder and gives its first entry; `_FILES$` with no argument then gives the
next, and `""` at the end.

## Syntax

<!-- begin generated: syntax -->
```text
_FILES$([fileSpec$])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `spec$`: a folder, a pattern such as `"*.tmp"`, or `""` for the current folder.

## Result

A STRING: an entry's name, a folder's ending with a backslash, or `""`.

## Behavior

`_FILES$(spec$)` gives the first entry, and each `_FILES$` after it the next, until `""`: a loop counts the two `.tmp`
files of a folder.
<!-- src: language/file-io "Listing a folder with _FILES$"; tests\reference\examples\_files-str_1 -->

A listing of a folder includes `.` and `..`.
<!-- src: language/file-io "Listing a folder with _FILES$" -->

## Errors

- `_FILES$` with no listing started, or after it gave `""`, raises run-time error 5.
  <!-- src: language/file-io "Listing a folder with _FILES$" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_files-str_1 -->
```qb64
MKDIR "_files_1_dir"
OPEN "_files_1_dir/a.tmp" FOR OUTPUT AS #1: CLOSE #1
OPEN "_files_1_dir/b.tmp" FOR OUTPUT AS #1: CLOSE #1
n = 0
f$ = _FILES$("_files_1_dir/*.tmp")
DO WHILE LEN(f$)
    n = n + 1
    f$ = _FILES$
LOOP
PRINT "Files found:"; n
KILL "_files_1_dir/a.tmp": KILL "_files_1_dir/b.tmp"
RMDIR "_files_1_dir"
END
```

Output:

```text
Files found: 2 

Press any key to continue
```
<!-- end generated -->

## See also

[`FILES`](files.md), [`_FILEEXISTS`](_fileexists.md), [`_DIREXISTS`](_fileexists.md).
