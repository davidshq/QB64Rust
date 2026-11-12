# VARPTR$

`VARPTR$(variable)` gives a string of three bytes: a type byte, then the variable's offset.

## Syntax

<!-- begin generated: syntax -->
```text
VARPTR(variable_name[(reference_index%)])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING of 3 bytes.

## Behavior

The type byte is 2 for an INTEGER and 3 for a STRING; the other two bytes are the offset, the low byte first.
<!-- src: language/conventional-memory "VARPTR$"; tests\corpus\slice\s158_varptr_strings; tests\reference\examples\varptr-str_1 -->

For a fixed-length string the offset is its descriptor's, and for a member its variable's.
<!-- src: language/conventional-memory "VARPTR$"; tests\corpus\slice\s158_varptr_strings -->

## Errors

- `VARPTR$` of an element of an array that is not of variable-length strings is QB64pe's error, and of a `STRING`
  member a compile error.
  <!-- src: language/conventional-memory "VARPTR$"; language/conventional-memory "What VARPTR and VARSEG accept" -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:varptr-str_1 -->
```qb64
DIM i AS INTEGER, s AS STRING
v$ = VARPTR$(i)
PRINT "VARPTR$ of an INTEGER:"; LEN(v$); "bytes, type"; ASC(v$, 1)
v$ = VARPTR$(s)
PRINT "VARPTR$ of a STRING:"; LEN(v$); "bytes, type"; ASC(v$, 1)
END
```

Output:

```text
VARPTR$ of an INTEGER: 3 bytes, type 2 
VARPTR$ of a STRING: 3 bytes, type 3 

Press any key to continue
```
<!-- end generated -->

## See also

`VARPTR`, `PLAY`, `DRAW`.
