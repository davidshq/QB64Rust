# _READFILE$

`_READFILE$(name$)` gives the whole content of a file as one string, and `_WRITEFILE name$, data$` writes a string as
the whole content of a file.

## Syntax

```text
content$ = _READFILE$(name$)
_WRITEFILE name$, data$
```

## Behavior

`_WRITEFILE` creates the file or replaces it, and `_READFILE$` gives back every byte.
<!-- src: language/file-io "Whole files and full paths"; tests\reference\examples\_readfile-str_1 -->

## Errors

- A file that cannot be found raises run-time error 53, a folder that cannot be found error 76; `_READFILE$` then
  gives `""`.
  <!-- src: language/file-io "Whole files and full paths" -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_readfile-str_1 -->
```qb64
_WRITEFILE "_readfile_1.tmp", "line one" + CHR$(10) + "line two"
d$ = _READFILE$("_readfile_1.tmp")
PRINT LEN(d$); "bytes:"
PRINT d$
KILL "_readfile_1.tmp"
END
```

Output:

```text
 17 bytes:
line one
line two

Press any key to continue
```
<!-- end generated -->

## See also

[`OPEN`](open.md), [`GET`](get.md), [`PUT`](put.md).
