# _INFLATE$

`_INFLATE$(s$[, size])` gives back a string that `_DEFLATE$` compressed. `size`, the length expected, makes it give
that many bytes.

## Syntax

<!-- begin generated: syntax -->
```text
_INFLATE$(stringToDecompress$[, originalSize&])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

`_INFLATE$(_DEFLATE$("hello world"), 3)` is `hel`. With `size` it gives that many bytes, the data first: 5000 for 1000
bytes of data, 5 for data cut short. A `size` of 0 or below gives the empty string.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_inflate-str_1 -->

Without `size`, a string that is not deflated data gives 10485760 zero bytes.
<!-- src: tests\corpus\slice\s109_data_processing -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _INFLATE$(5)`: "argument 1 of `_INFLATE$` needs a string".
  - `PRINT _INFLATE$("a", "b")`: "argument 2 of `_INFLATE$` needs a number".
  <!-- src: language/data-processing "Checksums, compression and base64"; verification\v33_x33_inflate_number; verification\v33_x46_inflate_string_size -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_inflate-str_1 -->
```qb64
c$ = _DEFLATE$("hello world")
PRINT "["; _INFLATE$(c$); "]"
PRINT "The first 3 bytes: ["; _INFLATE$(c$, 3); "]"
END
```

Output:

```text
[hello world]
The first 3 bytes: [hel]

Press any key to continue
```
<!-- end generated -->

## See also

[`_DEFLATE$`](_deflate-str.md).
