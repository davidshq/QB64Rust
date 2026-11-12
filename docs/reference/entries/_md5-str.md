# _MD5$

`_MD5$(s$)` gives the MD5 digest of a string as 32 upper-case hexadecimal digits.

## Syntax

<!-- begin generated: syntax -->
```text
_MD5$(dataString$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING of 32 characters.

## Behavior

`_MD5$("abc")` is `900150983CD24FB0D6963F7D28E17F72`, and of the empty string `D41D8CD98F00B204E9800998ECF8427E`.
<!-- src: language/data-processing "_MD5$"; verification\v49_c_md5_os; tests\reference\examples\_md5-str_1 -->

A program that names it links the part of libqb QB64pe links for it.
<!-- src: language/data-processing "_MD5$" -->

## Errors

- A form QB64pe rejects is a compile error, for example:
  - `PRINT _MD5$(5)`: "`_MD5$` needs a string".
  - `PRINT _MD5$`: "`_MD5$` takes 1 argument".
  <!-- src: language/data-processing "_MD5$"; verification\v49_x43_md5_number; verification\v49_x44_md5_bare -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_md5-str_1 -->
```qb64
PRINT _MD5$("abc")
PRINT _MD5$("")
END
```

Output:

```text
900150983CD24FB0D6963F7D28E17F72
D41D8CD98F00B204E9800998ECF8427E

Press any key to continue
```
<!-- end generated -->

## See also

`_CRC32`, `_ADLER32`.
