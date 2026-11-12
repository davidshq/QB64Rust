# _CRC32

`_CRC32(s$)` gives the CRC-32 checksum of a string.

## Syntax

<!-- begin generated: syntax -->
```text
_CRC32(dataString$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An `_UNSIGNED LONG`.

## Behavior

`_CRC32("abc")` is 891568578, and of the empty string 0.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_crc32_1 -->

`HEX$(_CRC32("The quick brown fox jumps over the lazy dog"))` is `414FA339`, the standard value.
<!-- src: tests\corpus\slice\s109_data_processing; tests\reference\examples\_crc32_1 -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/data-processing "Checksums, compression and base64" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_crc32_1 -->
```qb64
PRINT "CRC-32 of abc:"; _CRC32("abc")
PRINT "Of the fox: "; HEX$(_CRC32("The quick brown fox jumps over the lazy dog"))
PRINT "Of nothing:"; _CRC32("")
END
```

Output:

```text
CRC-32 of abc: 891568578 
Of the fox: 414FA339
Of nothing: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_ADLER32`, `_MD5$`.
