# _DEFINE

`_DEFINE` sets the type of names by their first letter, as `DEFINT` does, for any type a suffix can give:
`_DEFINE U AS _UNSIGNED _BYTE`.

## Syntax

```text
_DEFINE letters AS type
```

## Arguments

- `letters`: single letters and ranges, separated by commas, as for `DEFINT`.
- `type`: a numeric type with or without `_UNSIGNED`, `_OFFSET`, `_FLOAT`, `_BIT` or `_BIT * n`, `STRING`, or
  `STRING * n` with `n` above 0.

## Behavior

Names that start with the letters, and have no suffix and no `AS` clause, get the type: after
`_DEFINE u AS _UNSIGNED _BYTE`, `u = -1` stores 255.
<!-- src: language/default-types "_DEFINE"; verification\v24_a_define -->

The rules are those of `DEFINT`: from the statement on in the file, a later statement replaces an earlier one for
the same letter, and a suffix or `AS` wins. A `_DEFINE` after `DEFINT A-Z` changes only its own letters.
<!-- src: language/default-types "_DEFINE"; verification\v24_a_define -->

With `STRING * n`, each such name is a fixed-length string: after `_DEFINE a AS STRING * 5`, `a1 = "abcdefgh"`
holds `abcde`.
<!-- src: language/default-types "_DEFINE"; verification\v24_a_define; tests\reference\examples\_define_1 -->

## Errors

These are compile errors:

- A user type: "a default type cannot be a `TYPE`".
  <!-- src: language/default-types "_DEFINE"; verification\v24_x05_define_user_type -->
- A name that is no type: "the type `NOSUCH` of `_DEFINE` is no type".
  <!-- src: language/default-types "_DEFINE"; verification\v24_x07_define_unknown_type -->
- `_UNSIGNED` before a floating-point type: "`SINGLE` cannot be `_UNSIGNED`", with the type's name.
  <!-- src: language/default-types "_DEFINE"; verification\v24_x09_define_unsigned_single -->
- `STRING * 0`: "a fixed-length string's length must be 1 to 2147483647 (it is read in 32 bits)".
  <!-- src: language/default-types "_DEFINE"; verification\v24_x35_define_fixed_zero -->
- No `AS`: "expected `AS` and a type after the letters". No letters, as in `_DEFINE AS LONG`: "`_DEFINE` needs
  letters before `AS`".
  <!-- src: language/default-types "_DEFINE and DEFtype without their letters"; verification\v24_x10_define_no_as; verification\v24_x11_define_no_letters -->
- A second `AS` clause, as in `_DEFINE a AS INTEGER, b AS LONG`: "expected the end of the statement".
  <!-- src: verification\v24_x12_define_two_as -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:_define_1 -->
```qb64
_DEFINE U AS _UNSIGNED _BYTE
u = -1: PRINT "u ="; u
_DEFINE C AS STRING * 5
c = "abcdefgh": PRINT "["; c; "]"; LEN(c)
_DEFINE N-P AS _INTEGER64
n = 5000000000.5: PRINT "n ="; n
END
```

Output:

```text
u = 255 
[abcde] 5 
n = 5000000000 

Press any key to continue
```
<!-- end generated -->

## See also

[`DEFINT`](defint.md), [`_UNSIGNED`](_unsigned.md), [`STRING`](string.md).
