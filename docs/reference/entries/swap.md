# SWAP

`SWAP` exchanges the values of two variables of the same type: after `SWAP a, b`, `a` holds what `b` held and
`b` what `a` held.

## Syntax

```text
SWAP place1, place2
```

## Arguments

- The two places: variables, array elements or members of a user type.

## Behavior

The two values are exchanged. The places may be numbers of any type but `_BIT`, strings, or variables of one user
type.
<!-- src: language/builtin-statements "SWAP"; verification\v22_d_swap -->

Two numeric types that differ only in being signed count as the same, and their bits are exchanged: with
`l& = -1` and `u~& = 2`, `SWAP l&, u~&` leaves `l&` 2 and `u~&` 4294967295.
<!-- src: language/builtin-statements "SWAP"; verification\v22_x99_swap_signed_unsigned -->

Any two strings may be swapped, a fixed-length string among them. Each side is then cut or padded to its own
length: swapping `"long one"` with a `STRING * 3` holding `"abc"` gives `"abc"` and `"lon"`.
<!-- src: language/builtin-statements "SWAP"; verification\v22_x97_swap_fixed_lengths; tests\reference\examples\swap_1 -->

A place in parentheses is the place itself: `SWAP (a&), b&` swaps `a&` and `b&`.
<!-- src: language/builtin-statements "SWAP"; verification\v22_x103_swap_parens -->

An element whose index is out of range raises error 9. After it, the exchange is made with the array's first
element, as in QB64pe.
<!-- src: language/builtin-statements "SWAP" -->

## Errors

These are compile errors:

- Two places of different types, as `SWAP a&, d#` or a string with a number: "`SWAP` needs two variables of the
  same type". An INTEGER and a LONG are different types.
  <!-- src: language/builtin-statements "SWAP"; verification\v22_x91_swap_long_double; verification\v22_x96_swap_string_number; verification\v22_x98_swap_integer_long -->
- A `_BIT` variable or element: "`SWAP` cannot exchange `_BIT` variables".
  <!-- src: language/builtin-statements "SWAP"; verification\v22_x135_swap_bit; verification\v24_x82_bit_array_swap -->
- A number or an expression in place of a variable: "`SWAP` needs a variable here". A constant: "cannot assign to
  the constant `c`".
  <!-- src: language/builtin-statements "SWAP"; verification\v22_x92_swap_literal; verification\v22_x93_swap_expression; verification\v22_x102_swap_const -->
- Only one place: "expected `,` between the two variables".
  <!-- src: verification\v22_x94_swap_one -->
- The name of the FUNCTION it stands in: "the FUNCTION's own name cannot be a target of `SWAP`".
  <!-- src: language/builtin-statements "SWAP"; verification\v22_x107_swap_function_result -->

## Differences

On Linux, QB64pe finds two places whose indexes call a FUNCTION in the other order, right to left. QB64Rust goes
left to right everywhere, as QB64pe does on Windows (`DIVERGENCES-QB64PE.md` D-054).
<!-- src: D-054 -->

## Availability

<!-- begin generated: availability -->
Supported. All 15 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:swap_1 -->
```qb64
a = 1: b = 2
SWAP a, b
PRINT "a ="; a; "and b ="; b
DIM names(1 TO 2) AS STRING
names(1) = "Ney": names(2) = "Murat"
SWAP names(1), names(2)
PRINT names(1); " "; names(2)
DIM code AS STRING * 3
s$ = "long one": code = "abc"
SWAP s$, code
PRINT "["; s$; "] ["; code; "]"
END
```

Output:

```text
a = 2 and b = 1 
Murat Ney
[abc] [lon]

Press any key to continue
```
<!-- end generated -->

## See also

`LET`, `DIM`.
