# DECLARE

`DECLARE SUB` and `DECLARE FUNCTION` lines announce a procedure before its definition. QuickBASIC needed them; in
QB64Rust they are accepted and change nothing. `DECLARE LIBRARY`, for C functions, has its own page.

## Syntax

```text
DECLARE SUB name [(parameter [, parameter]...)]
DECLARE FUNCTION name[suffix] [(parameter [, parameter]...)]

parameter:  name [AS type]  |  name AS ANY
```

## Behavior

A well-formed `DECLARE` line is accepted and otherwise ignored. Calls are checked against the procedure's
definition, not against the `DECLARE`: a `DECLARE` that disagrees with the definition, or names a procedure that
does not exist, is not an error.
<!-- src: language/procedures "DECLARE lines"; tests\reference\examples\declare_1 -->

`AS ANY` may stand for a parameter's type, as in QuickBASIC programs: `DECLARE SUB inner (z AS ANY)`.
<!-- src: language/procedures "DECLARE lines"; verification\v61_p1_type_chain -->

`ALIAS name` in a `DECLARE` line outside `DECLARE LIBRARY` is accepted and changes nothing.
<!-- src: language/procedures "ALIAS in a DECLARE line"; verification\v57_a01_alias_quoted -->

An `AS type` after a stray comma, `idx, AS LONG`, is taken as the clause of the parameter before it.
<!-- src: language/procedures "DECLARE lines"; verification\v70_e1_declare_stray_comma -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:declare_1 -->
```qb64
DECLARE SUB Report (title AS STRING, n AS INTEGER)
DECLARE FUNCTION Square& (x AS LONG)
Report "Squares", 3
END

SUB Report (title AS STRING, n AS INTEGER)
    PRINT title
    FOR i& = 1 TO n
        PRINT i&; Square&(i&)
    NEXT i&
END SUB

FUNCTION Square& (x AS LONG)
    Square& = x * x
END FUNCTION
```

Output:

```text
Squares
 1  1 
 2  4 
 3  9 

Press any key to continue
```
<!-- end generated -->

## See also

[`SUB`](sub.md), [`FUNCTION`](function.md), [`DECLARE LIBRARY`](declare-library.md).
