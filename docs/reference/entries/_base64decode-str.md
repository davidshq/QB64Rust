# _BASE64DECODE$

`_BASE64DECODE$(s$)` decodes a base64 string.

## Syntax

<!-- begin generated: syntax -->
```text
_BASE64DECODE$(stringToDecode$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

`YWJj` decodes to `abc` and `YQ==` to `a`.
<!-- src: language/data-processing "Checksums, compression and base64"; tests\corpus\slice\s109_data_processing; tests\reference\examples\_base64decode-str_1 -->

A string whose length is not a multiple of 4, such as `YQ` without its padding, or that holds a character outside
base64, gives the empty string.
<!-- src: tests\corpus\slice\s109_data_processing; tests\reference\examples\_base64decode-str_1 -->

A program that names it is linked with libqb's data-processing part, as QB64pe's build is.
<!-- src: language/data-processing "The runtime part of the data-processing functions" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _BASE64DECODE$(5)`: "`_BASE64DECODE$` needs a string".
  - `PRINT _BASE64DECODE$`: "`_BASE64DECODE$` takes 1 argument".
  <!-- src: language/data-processing "Checksums, compression and base64"; verification\v71_x01_base64decode_number; verification\v71_x02_base64decode_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_base64decode-str_1 -->
```qb64
PRINT "["; _BASE64DECODE$("YWJj"); "]["; _BASE64DECODE$("YQ=="); "]"
PRINT "Without its padding: ["; _BASE64DECODE$("YQ"); "]"
PRINT "Not base64: ["; _BASE64DECODE$("!!!!"); "]"
END
```

Output:

```text
[abc][a]
Without its padding: []
Not base64: []

Press any key to continue
```
<!-- end generated -->

## See also

[`_BASE64ENCODE$`](_base64encode-str.md).
