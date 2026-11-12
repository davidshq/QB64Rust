# LSET

`LSET s$ = value` puts a value at the left of a string without changing its length, padding with blanks or cutting the
end. `RSET` puts it at the right.

## Syntax

```text
LSET string$ = value$
RSET string$ = value$
```

## Behavior

A shorter value is padded with blanks; a longer one is cut at its end, by both. With `s$` 8 bytes long, `LSET s$ =
"ab"` makes `"ab      "` and `RSET s$ = "ab"` makes `"      ab"`.
<!-- src: language/record-files "LSET and RSET"; tests\reference\examples\lset_1 -->

They are how a `FIELD` string is set without leaving the record buffer.
<!-- src: language/record-files "FIELD" -->

The target may be a variable-length or fixed-length string, an element or a member.
<!-- src: language/record-files "LSET and RSET" -->

## Errors

- A target that is no string, a `TYPE` value among them, is a compile error.
  <!-- src: language/record-files "LSET and RSET" -->

## Differences

From QuickBASIC 4.5: `LSET r = r2` of two `TYPE` variables copies their bytes there. QB64Rust rejects it, as QB64pe
does (`DIVERGENCES-QB45.md` Q-013).
<!-- src: Q-013 -->

## Availability

<!-- begin generated: availability -->
Supported. All 11 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:lset_1 -->
```qb64
s$ = "12345678"
LSET s$ = "ab"
PRINT "["; s$; "]"
RSET s$ = "ab"
PRINT "["; s$; "]"
LSET s$ = "much too long"
PRINT "["; s$; "]"
END
```

Output:

```text
[ab      ]
[      ab]
[much too]

Press any key to continue
```
<!-- end generated -->

## See also

[`FIELD`](field.md), [`MID$`](mid-str.md).
