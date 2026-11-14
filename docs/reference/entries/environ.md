# ENVIRON

`ENVIRON "name=value"` sets an environment variable of the program, and `ENVIRON$(name$)` gives one. `ENVIRON$(n)`
gives the nth entry, as `name=value`.

## Syntax

<!-- begin generated: syntax -->
```text
ENVIRON stringExpression$
ENVIRON$(listIndex%)
ENVIRON$(systemID$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
value$ = ENVIRON$(name$)
entry$ = ENVIRON$(n)
```

## Behavior

After `ENVIRON "QBREF_EXAMPLE=abc"`, `ENVIRON$("QBREF_EXAMPLE")` is `abc`. A name not set gives "".
<!-- src: language/builtin-statements "SHELL and ENVIRON"; verification\v22_d_environ; tests\reference\examples\environ_1 -->

`ENVIRON$(n)` past the last entry gives "". `_ENVIRONCOUNT` is the number of entries.
<!-- src: tests\reference\examples\_environcount_1 -->

## Errors

- `ENVIRON` with a number or two arguments, and `ENVIRON$` bare, empty or with two, are compile errors.
  <!-- src: verification\v22_x08_environ_number; verification\v22_x19_environ_two; verification\v22_x170_environ_bare; verification\v22_x172_environ_empty_parens -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:environ_1 -->
```qb64
ENVIRON "QBREF_EXAMPLE=abc"
PRINT "Set: ["; ENVIRON$("QBREF_EXAMPLE"); "]"
PRINT "Not set: ["; ENVIRON$("QBREF_NO_SUCH_NAME"); "]"
PRINT "The first entry has an = in it:"; INSTR(ENVIRON$(1), "=") > 0
END
```

Output:

```text
Set: [abc]
Not set: []
The first entry has an = in it:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`_ENVIRONCOUNT`](_environcount.md), [`SHELL`](shell.md), [`COMMAND$`](command-str.md).
