# SUB

`SUB … END SUB` defines a procedure: a named block of statements that the program runs by naming it, with
arguments for its parameters. A SUB gives no value; a `FUNCTION` does.

## Syntax

```text
SUB name [(parameter [, parameter]...)] [STATIC]
    statements
END SUB

parameter:  name[suffix] [AS type]  |  name() [AS type]
```

## Behavior

A SUB may be defined before or after the main module's code, and called from anywhere in the program. Procedures
cannot be nested.
<!-- src: language/procedures "Procedure definitions"; tests\reference\examples\sub_1 -->

A parameter has the type of its `AS` clause or suffix, or else the default type of its name. It may be a number of
any type, `_BIT` included, a `STRING`, an array (`name()`) or a user type.
<!-- src: language/procedures "Procedure definitions"; language/procedures "Array parameters"; language/procedures "User-type parameters" -->

A variable passed plainly, with exactly the parameter's type, is passed by reference: the SUB's stores to the
parameter change it. In parentheses, or as an expression, it is passed as a copy whose changes are lost: after
`Bump n&` the value has changed, and after `Bump (n&)` it has not.
<!-- src: language/procedures "Argument passing"; tests\reference\examples\sub_1 -->

A string variable is passed by reference, also in parentheses. An integer of the parameter's width but the other
signedness is passed by reference too, and read with the parameter's type.
<!-- src: language/procedures "Argument passing"; verification\v21_b_passing -->

An argument of another type is converted as an assignment converts it: 2.5 passed to an INTEGER parameter is 2.
<!-- src: language/procedures "Argument passing" -->

An array parameter takes a whole array, `a()`, by reference. A user-type parameter takes a variable, element or
member of that type by reference.
<!-- src: language/procedures "Array parameters"; language/procedures "User-type parameters" -->

Inside a SUB, a name means a parameter, a `STATIC` variable, a `SHARED` variable or a main-module `DIM SHARED`
variable declared above the SUB, in that order; otherwise it is a local, new on each call. `STATIC` after the header
keeps every local between calls.
<!-- src: language/procedures "Procedure scopes"; Q-010 -->

`EXIT SUB` returns at once. A SUB and a FUNCTION may share a name: the name as a statement calls the SUB, and as a
value the FUNCTION.
<!-- src: language/procedures "Calls"; language/procedures "Procedure definitions"; verification\v70_a1_func_then_sub -->

A line that is a SUB's name and a colon is a call of the SUB, unless a jump of the program names the word.
<!-- src: language/control-flow "A SUB's name and a colon, with no jump to the name" -->

## Errors

These are compile errors:

- A call with the wrong number of arguments: "`s` takes 1 argument, not 2".
  <!-- src: language/procedures "Calls" -->
- A name that is no SUB, as a statement: "`nosuch` is no statement and no SUB".
  <!-- src: language/procedures "A statement that names no SUB" -->
- A SUB defined inside another: "a SUB or FUNCTION cannot be defined inside another; expected `END SUB` before it".
  <!-- src: language/procedures "Procedure definitions" -->
- Two SUBs of one name: "name already in use: `s`".
  <!-- src: language/procedures "Procedure definitions" -->
- A parameter named after a keyword or a built-in, such as `LEN` or `CLS`.
  <!-- src: language/procedures "Reserved names" -->
- `EXIT SUB` in the main module: "`EXIT SUB` must be inside a SUB or FUNCTION".
  <!-- src: language/procedures "Calls" -->

## Differences

From QB64pe: the reference QB64pe fails its own C++ build for a `_BIT` parameter. QB64Rust accepts one, as later
QB64pe does (`DIVERGENCES-QB64PE.md` D-057).
<!-- src: D-057 -->

## Availability

<!-- begin generated: availability -->
Supported. All 348 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:sub_1 -->
```qb64
n& = 1
Bump n&
PRINT "After Bump n&, n& ="; n&
Bump (n&)
PRINT "After Bump (n&), n& ="; n&
Greet "Ney", 2
END

SUB Bump (x AS LONG)
    x = x + 1
END SUB

SUB Greet (who AS STRING, times AS INTEGER)
    FOR i = 1 TO times
        PRINT "Hello, "; who
    NEXT i
END SUB
```

Output:

```text
After Bump n&, n& = 2 
After Bump (n&), n& = 2 
Hello, Ney
Hello, Ney

Press any key to continue
```
<!-- end generated -->

## See also

[`FUNCTION`](function.md), [`CALL`](call.md), [`DECLARE`](declare.md), [`SHARED`](shared.md), [`STATIC`](static.md), [`EXIT`](exit.md).
