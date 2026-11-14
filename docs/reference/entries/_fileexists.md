# _FILEEXISTS

`_FILEEXISTS(name$)` tells whether a file exists, and `_DIREXISTS(path$)` whether a folder does: -1 when it does, 0
when it does not.

## Syntax

<!-- begin generated: syntax -->
```text
_DIREXISTS(path$)
_FILEEXISTS(fileName$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The file's or folder's name.

## Result

-1 or 0.

## Behavior

`_FILEEXISTS` is -1 for a file that exists and 0 otherwise; `_DIREXISTS(".")` is -1.
<!-- src: language/file-io "File functions"; tests\reference\examples\_fileexists_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_fileexists_1 -->
```qb64
OPEN "_fileexists_1.tmp" FOR OUTPUT AS #1: CLOSE #1
PRINT _FILEEXISTS("_fileexists_1.tmp"); _FILEEXISTS("no such file.tmp")
PRINT _DIREXISTS("."); _DIREXISTS("no such folder")
KILL "_fileexists_1.tmp"
END
```

Output:

```text
-1  0 
-1  0 

Press any key to continue
```
<!-- end generated -->

## See also

[`KILL`](kill.md), [`OPEN`](open.md), [`_FILES$`](_files-str.md).
