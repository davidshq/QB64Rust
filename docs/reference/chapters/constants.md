# Constants

`CONST` names a value computed when the program is compiled. Every program also has the constants of QB64pe's
auto-included file, and `$COLOR` adds the color names.

The pages of this area: [CONST](../entries/const.md), [$COLOR](../entries/meta-color.md),
[OPTION](../entries/option.md).

## Values

A constant is computed as QB64pe's constant evaluator computes it: integer arithmetic in 64 bits, `/` in floating
point, `^` from right to left. An integer result is an `_INTEGER64`.
<!-- src: language/constants "CONST values"; tests\reference\examples\const_1 -->

A `CONST` may call the functions of QB64pe's evaluator's list. Where the value of a math function depends on the
machine's library, the constant takes a correctly rounded value.
<!-- src: language/constants "Functions in a CONST"; D-026 -->

A chain of `^` without parentheses gets a warning: a `CONST` computes it from right to left, a statement from left to
right.
<!-- src: language/constants "Warning for a chained power in a CONST"; tests\reference\examples\const_1 -->

## Where a constant is known

A constant is known from its line to the end of its body. The main module's constants are known in the procedures
below them.
<!-- src: language/constants "CONST visibility" -->

The constants of `beforefirstline.bi` are known everywhere, as if their lines stood above the program's first line;
[the appendix](../appendix-constants.md) lists them.
<!-- src: language/constants "Constants of the auto-included file" -->

`OPTION _EXPLICIT` makes every variable used without a declaration a compile error, in the whole program.
<!-- src: language/constants "OPTION _EXPLICIT" -->
