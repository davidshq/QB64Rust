# SHARED

`SHARED` inside a SUB or FUNCTION makes a variable of the main module visible there: `SHARED gold AS LONG`. In the
main module, `DIM SHARED` does the same for every procedure below it.

## Syntax

```text
SHARED name [AS type] [, name [AS type]]...
DIM SHARED name [AS type] [, ...]
```

## Behavior

In a procedure, `SHARED name AS type` names the main module's variable of that name and type. If the main module
has none, it is made.
<!-- src: language/procedures "Procedure scopes"; tests\reference\examples\shared_1 -->

The type counts: `SHARED g` without `AS` names the main module's `g` of the default type, a SINGLE. If the main
module declared `g` as a LONG, that is another variable, and the procedure sees 0.
<!-- src: language/procedures "Procedure scopes" -->

A `SHARED name AS type` in a procedure that comes first in the file gives the main module's `name` that type: the
main module's `h = 9.5` then stores 10 into the LONG `h`.
<!-- src: language/procedures "Procedure scopes" -->

`DIM SHARED` in the main module makes the variable visible in every procedure that comes later in the file, with
no `SHARED` line in them. A procedure above the `DIM SHARED` does not see it.
<!-- src: language/procedures "Procedure scopes"; tests\reference\examples\shared_1 -->

A `DIM` or `REDIM` in a procedure that makes a local of the name and type of a `DIM SHARED` variable gets the
warning "this `score` is a new local variable, which hides the `DIM SHARED` one of the main module".
<!-- src: compiler/lints "Local hiding a shared variable" -->

## Errors

These are compile errors:

- `SHARED` in the main module: "`SHARED` can stand only in a SUB or FUNCTION".
  <!-- src: language/procedures "SHARED in the main module" -->
- Under `OPTION _EXPLICIT`, a `SHARED` of a main-module variable that is declared only below the procedure:
  "variable `w` (LONG) is not declared (`OPTION _EXPLICIT`)".
  <!-- src: language/constants "OPTION _EXPLICIT"; verification\v17_f_explicit_shared_before_dim -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:shared_1 -->
```qb64
DIM SHARED turn AS INTEGER
DIM gold AS LONG
gold = 500
NextTurn
NextTurn
PRINT "After two turns: turn ="; turn; "and gold ="; gold
END

SUB NextTurn
    SHARED gold AS LONG
    turn = turn + 1
    gold = gold - 100
END SUB
```

Output:

```text
After two turns: turn = 2 and gold = 300 

Press any key to continue
```
<!-- end generated -->

## See also

`DIM`, `STATIC`, `COMMON`, `SUB`.
