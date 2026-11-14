# Procedures

A program is a main module and its SUBs and FUNCTIONs. A procedure has parameters, its own variables and labels, and
reaches the main module's variables only when they are shared.

The pages of this area: [SUB](../entries/sub.md), [FUNCTION](../entries/function.md), [CALL](../entries/call.md),
[DECLARE](../entries/declare.md), [SHARED](../entries/shared.md), [STATIC](../entries/static.md),
[DIM](../entries/dim.md).

## Defining and calling

`SUB name (params)` and `FUNCTION name (params)` may stand before or after the main module's code. A SUB is called
with `CALL name(args)` or `name args`; a FUNCTION in an expression, and inside it, assigning to its name sets the
result.
<!-- src: language/procedures "Procedure definitions"; language/procedures "Calls"; tests\reference\examples\call_1 -->

`DECLARE SUB` and `DECLARE FUNCTION` lines are accepted and change nothing; calls are checked against the definition.
<!-- src: language/procedures "DECLARE lines" -->

## Arguments

A variable written plainly, of exactly the parameter's type, is passed by reference: the procedure's stores change it.
Anything else, a variable in parentheses included, is passed as a copy.
<!-- src: language/procedures "Argument passing"; tests\reference\examples\sub_1 -->

An array parameter `a()` takes a whole array, and a user-type parameter a place of that type, both by reference.
<!-- src: language/procedures "Array parameters"; language/procedures "User-type parameters" -->

## Names inside a procedure

Inside a procedure a name is, in this order: a parameter, the FUNCTION's result, a `STATIC` variable, a `SHARED` one,
a main-module variable declared with `DIM SHARED` earlier in the file, and otherwise a local variable.
<!-- src: language/procedures "Procedure scopes" -->

A variable or procedure may not take the name of a keyword or built-in without a suffix.
<!-- src: language/procedures "Reserved names" -->
