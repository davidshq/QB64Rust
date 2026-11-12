# FUNCTION

`FUNCTION … END FUNCTION` defines a procedure that gives a value. The program calls it in an expression, as
`Twice&(21)`, and the function sets its result by assigning to its own name.

## Syntax

```text
FUNCTION name[suffix] [(parameter [, parameter]...)] [STATIC]
    statements
    name = value
END FUNCTION
```

## Behavior

The result's type is given by the name's suffix, of any numeric type or `$`, or else by the default type of the
name. `name$n` gives a string of exactly n bytes, cut or padded with spaces.
<!-- src: language/procedures "Procedure definitions"; tests\reference\examples\function_1 -->

Inside the FUNCTION, assigning to its name, with or without the suffix, sets the result. Any other use of the name is
a call. A FUNCTION left before its result is set, by `EXIT FUNCTION` or its end, returns 0 or an empty string.
<!-- src: language/procedures "Calls" -->

A FUNCTION without parameters is called by its name alone. It may be called above its definition in the file.
<!-- src: language/procedures "Calls"; language/procedures "Procedure definitions" -->

The parameters work as a SUB's do: a variable passed plainly with the parameter's type is passed by reference.
<!-- src: language/procedures "Argument passing" -->

A FUNCTION's result is printed with its own type: `PRINT twice&(n)` prints a LONG.
<!-- src: language/procedures "Calls" -->

Each call's result is its own value, also for two calls of a `$n` FUNCTION in one expression.
<!-- src: language/procedures "Procedure definitions"; D-014 -->

## Errors

These are compile errors:

- A string argument for a number parameter, or the reverse: "a number is required for this parameter of a
  FUNCTION".
  <!-- src: language/procedures "Calls" -->
- A FUNCTION used as a statement: "the FUNCTION `f` cannot stand as a statement".
  <!-- src: language/procedures "A program FUNCTION as a statement" -->
- A user type as the result.
  <!-- src: language/procedures "User-type parameters" -->
- A call of a `$n` FUNCTION with another length, as `f$4` for `f$5`.
  <!-- src: language/procedures "Procedure definitions"; verification\v21_x49_fixed_function_length -->

## Differences

From QB64pe: two calls of a `$n` FUNCTION in one expression share their result there, so `fs$5("ab") + fs$5("cd")`
gives `cd   cd   `. QB64Rust gives `ab   cd   ` (`DIVERGENCES-QB64PE.md` D-014).
<!-- src: D-014 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:function_1 -->
```qb64
PRINT "Twice 21 is"; Twice&(21)
PRINT "The larger of 3 and 8 is"; Larger(3, 8)
PRINT "["; Pad$("ab", 5); "]"
END

FUNCTION Twice& (a AS LONG)
    Twice& = a * 2
END FUNCTION

FUNCTION Larger (a, b)
    IF a > b THEN Larger = a ELSE Larger = b
END FUNCTION

FUNCTION Pad$ (s AS STRING, w AS INTEGER)
    Pad$ = s + SPACE$(w - LEN(s))
END FUNCTION
```

Output:

```text
Twice 21 is 42 
The larger of 3 and 8 is 8 
[ab   ]

Press any key to continue
```
<!-- end generated -->

## See also

`SUB`, `DECLARE`, `EXIT`, `STATIC`.
