# The order of evaluation

Where the order in which parts of a statement are computed could change what a program does, QB64Rust computes them in
the order QB64pe's Windows build does, on every platform.

The pages of this area: [_ANDALSO](../entries/_andalso.md), [_ORELSE](../entries/_orelse.md),
[_IIF](../entries/_iif.md).

## The order kept

The arguments of a call, the operands of an operator, the indexes of an element and the two sides of a string
assignment are computed in the order of QB64pe's C++, which is not always left to right.
<!-- src: language/evaluation-order "Parts are evaluated in the Windows build's order on every platform" -->

The order matters where a part calls a FUNCTION or a built-in that changes the program's state, or where two parts can
each raise an error.
<!-- src: language/evaluation-order "Which parts the order applies to" -->

QB64pe's Linux build often computes them right to left instead, so one program can print other results there:
`_ASSERT f(1), g$("never")` calls `g$` before `f` on Linux. QB64Rust's Linux build keeps the Windows order
(`DIVERGENCES-QB64PE.md` D-060).
<!-- src: D-060 -->

## Parts computed only when needed

The right side of `_ANDALSO` and `_ORELSE`, and the branch of `_IIF` not chosen, are computed only when needed. Each
item of a `PRINT` is computed after the one before it is printed.
<!-- src: language/evaluation-order "Orders the C++ already fixes stay as they are" -->
