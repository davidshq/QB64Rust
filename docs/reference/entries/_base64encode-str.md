# _BASE64ENCODE$

`_BASE64ENCODE$(s$)` encodes a string in base64, with `=` to pad the last group.

## Syntax

<!-- begin generated: syntax -->
```text
_BASE64ENCODE$(stringToEncode$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

`a`, `ab` and `abc` encode to `YQ==`, `YWI=` and `YWJj`, the empty string to the empty string. Any byte may be
encoded.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_base64encode-str_1 -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error.
  <!-- src: language/data-processing "Checksums, compression and base64" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_base64encode-str_1 -->
```qb64
PRINT "["; _BASE64ENCODE$(""); "]["; _BASE64ENCODE$("a"); "]["; _BASE64ENCODE$("ab"); "]["; _BASE64ENCODE$("abc"); "]"
PRINT _BASE64ENCODE$(CHR$(0) + CHR$(255) + CHR$(254))
END
```

Output:

```text
[][YQ==][YWI=][YWJj]
AP/+

Press any key to continue
```
<!-- end generated -->

## See also

`_BASE64DECODE$`.
