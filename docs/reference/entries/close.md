# CLOSE

`CLOSE` closes files: `CLOSE #1` closes file 1, and `CLOSE` alone closes every open file.

## Syntax

```text
CLOSE [[#]filenumber [, [#]filenumber]...]
```

## Behavior

`CLOSE` closes every open file, and `CLOSE #n, #m` the files named. What was written is then in the file.
<!-- src: language/file-io "OPEN and CLOSE"; tests\reference\examples\close_1 -->

Closing a number that is not open is no error.
<!-- src: language/file-io "OPEN and CLOSE"; tests\reference\examples\close_1 -->

`RESET` closes every file too.
<!-- src: language/builtin-functions "The old compiler's stubs" -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:close_1 -->
```qb64
OPEN "close_1.tmp" FOR OUTPUT AS #1
OPEN "close_1b.tmp" FOR OUTPUT AS #2
CLOSE #1, #2
CLOSE #7
PRINT "Closing a file that is not open is no error."
KILL "close_1.tmp": KILL "close_1b.tmp"
END
```

Output:

```text
Closing a file that is not open is no error.

Press any key to continue
```
<!-- end generated -->

## See also

`OPEN`, `RESET`.
