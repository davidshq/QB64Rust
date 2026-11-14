# _STRCMP

`_STRCMP` compares two strings and gives -1, 0 or 1: below, equal or above. `_STRICMP` does the same without regard
to the letters' case.

## Syntax

<!-- begin generated: syntax -->
```text
_STRCMP(string1$, string2$)
_STRICMP(string1$, string2$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A LONG: -1 when the first string sorts before the second, 0 when they are equal, 1 when it sorts after.

## Behavior

`_STRCMP` compares byte by byte: `_STRCMP("a", "b")` is -1 and `_STRCMP("b", "a")` is 1. Upper-case letters sort
before lower-case ones, so `_STRCMP("a", "A")` is 1. A string that is the start of the other sorts first:
`_STRCMP("", "a")` is -1 and `_STRCMP("ab", "a")` is 1.
<!-- src: verification\v22_f_functions; tests\reference\examples\_strcmp_1 -->

`_STRICMP` compares without regard to case: `_STRICMP("a", "A")` is 0, and `_STRICMP("AB", "a")` is 1.
<!-- src: verification\v22_f_functions -->

## Errors

These are compile errors:

- A number as an argument: "argument 1 of `_STRCMP` needs a string".
  <!-- src: verification\v22_x176_strcmp_number -->
- One argument: "`_STRCMP` takes 2 arguments".
  <!-- src: verification\v22_x177_strcmp_one -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_strcmp_1 -->
```qb64
PRINT _STRCMP("apple", "banana"); _STRCMP("b", "a"); _STRCMP("a", "a")
PRINT _STRCMP("a", "A"); _STRICMP("a", "A")
END
```

Output:

```text
-1  1  0 
 1  0 

Press any key to continue
```
<!-- end generated -->

## See also

[`UCASE$`](ucase-str.md), [`LCASE$`](lcase-str.md), [`INSTR`](instr.md).
