# LET

`LET` starts an assignment: `LET x = 5` is the same as `x = 5`. It is optional and seldom written.

## Syntax

```text
[LET] variable = expression
[LET] array() = array()
```

## Behavior

`LET variable = expression` stores the value in the variable, as the same line without `LET` does.
<!-- src: tests\corpus\runtime_comparison\105_let; tests\corpus\slice\s06_print_items -->

`LET` may also start a whole-array assignment, `LET b%() = a%()`, which copies every element of `a%()` into
`b%()`.
<!-- src: language/arrays-and-types "Whole-array assignment"; verification\v36_a_numbers -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:let_1 -->
```qb64
LET count = 5
count = count + 1
PRINT "count ="; count
LET greeting$ = "Hello"
PRINT greeting$
END
```

Output:

```text
count = 6 
Hello

Press any key to continue
```
<!-- end generated -->

## See also

`SWAP`, `DIM`, `CONST`.
