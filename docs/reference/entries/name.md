# NAME

`NAME old$ AS new$` renames a file or moves it to another folder.

## Syntax

```text
NAME oldname$ AS newname$
```

## Behavior

After `NAME "a.tmp" AS "b.tmp"`, the file is found as `b.tmp` and no longer as `a.tmp`.
<!-- src: language/file-io "File-system statements"; tests\reference\examples\name_1 -->

It raises QB64pe's errors, as for a file that does not exist.
<!-- src: language/file-io "File-system statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:name_1 -->
```qb64
OPEN "name_1.tmp" FOR OUTPUT AS #1: CLOSE #1
NAME "name_1.tmp" AS "name_1b.tmp"
PRINT _FILEEXISTS("name_1.tmp"); _FILEEXISTS("name_1b.tmp")
KILL "name_1b.tmp"
END
```

Output:

```text
 0 -1 

Press any key to continue
```
<!-- end generated -->

## See also

[`KILL`](kill.md), [`_FILEEXISTS`](_fileexists.md).
