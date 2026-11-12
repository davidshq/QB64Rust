# _ADLER32

`_ADLER32(s$)` gives the Adler-32 checksum of a string.

## Syntax

<!-- begin generated: syntax -->
```text
_ADLER32(dataString$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An `_UNSIGNED LONG`.

## Behavior

`_ADLER32("abc")` is 38600999, `HEX$(_ADLER32("Wikipedia"))` is `11E60398`, and of the empty string 1.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_adler32_1 -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _ADLER32(5)`: "`_ADLER32` needs a string".
  - `PRINT _ADLER32`: "`_ADLER32` takes 1 argument".
  <!-- src: language/data-processing "Checksums, compression and base64"; verification\v33_x30_adler32_number; verification\v33_x35_adler32_alone -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_adler32_1 -->
```qb64
PRINT "Adler-32 of abc:"; _ADLER32("abc")
PRINT "Of Wikipedia: "; HEX$(_ADLER32("Wikipedia"))
PRINT "Of nothing:"; _ADLER32("")
END
```

Output:

```text
Adler-32 of abc: 38600999 
Of Wikipedia: 11E60398
Of nothing: 1 

Press any key to continue
```
<!-- end generated -->

## See also

`_CRC32`, `_DEFLATE$`.
