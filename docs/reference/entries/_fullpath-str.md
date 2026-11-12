# _FULLPATH$

`_FULLPATH$(path$)` gives the full path of an existing file or folder.

## Syntax

<!-- begin generated: syntax -->
```text
_FULLPATH$(pathName$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `path$`: a file or folder, relative or full.

## Result

A STRING.

## Behavior

`_FULLPATH$` gives the whole path, ending with the name given.
<!-- src: language/file-io "Whole files and full paths"; tests\reference\examples\_fullpath-str_1 -->

An empty path gives `""` with no error.
<!-- src: language/file-io "Whole files and full paths" -->

## Errors

- A path that does not exist raises run-time error 76, and the result is `""`.
  <!-- src: language/file-io "Whole files and full paths" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_fullpath-str_1 -->
```qb64
OPEN "_fullpath_1.tmp" FOR OUTPUT AS #1: CLOSE #1
p$ = _FULLPATH$("_fullpath_1.tmp")
PRINT "The full path ends with the name: "; RIGHT$(p$, 15)
PRINT "It is longer than the name:"; LEN(p$) > 15
KILL "_fullpath_1.tmp"
END
```

Output:

```text
The full path ends with the name: _fullpath_1.tmp
It is longer than the name:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`_CWD$`, `_FILEEXISTS`.
