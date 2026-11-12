# _DEFLATE$

`_DEFLATE$(s$[, level])` compresses a string with zlib's deflate, and `_INFLATE$` gives it back.

## Syntax

<!-- begin generated: syntax -->
```text
_DEFLATE$(stringToCompress$[, compressionLevel&])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

1000 bytes of `a` deflate to 24, and `_INFLATE$` of the result is the string again.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_deflate-str_1 -->

`level` is zlib's, from 0 (stored, not compressed) to 9. Any other value, such as -2 or 12, compresses as the default
does.
<!-- src: tests\corpus\slice\s109_data_processing; tests\reference\examples\_deflate-str_1 -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/data-processing "Checksums, compression and base64" -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_deflate-str_1 -->
```qb64
s$ = STRING$(1000, "a")
c$ = _DEFLATE$(s$)
PRINT "1000 bytes deflated to"; LEN(c$)
PRINT "Inflated back:"; _INFLATE$(c$) = s$
PRINT "At level 0, stored:"; LEN(_DEFLATE$(s$, 0))
END
```

Output:

```text
1000 bytes deflated to 24 
Inflated back:-1 
At level 0, stored: 1011 

Press any key to continue
```
<!-- end generated -->

## See also

`_INFLATE$`, `_BASE64ENCODE$`.
