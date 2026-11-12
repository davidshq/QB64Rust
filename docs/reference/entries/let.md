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

`LET` may also start a whole-array assignment, `LET b() = a()`, which copies every element of `a()` into `b()`.
<!-- src: language/arrays-and-types "Whole-array assignment"; verification\v36_a_numbers -->

With a type suffix on both names, `LET b%() = a%()` copies only the first element, as QB64pe does: `b%(1)` is not
changed.
<!-- src: language/arrays-and-types "Whole-array assignment checks"; verification\v36_a_numbers -->

A whole-array assignment is deep: string elements and members are the target's own afterwards. The target keeps its
bounds, and the same array on both sides changes nothing.
<!-- src: language/arrays-and-types "Whole-array assignment" -->

## Errors

- A whole-array assignment raises error 9 and copies nothing when either array is not allocated, or when a dimension
  of the two has another lower bound or another number of elements. A static array after `ERASE` counts as
  allocated.
  <!-- src: language/arrays-and-types "Whole-array assignment at run time" -->
- These are compile errors: arrays of another element type, "cannot assign an array of `LONG` to an array of
  `INTEGER`"; another number of dimensions, "cannot assign an array of 1 dimension to an array of 2 dimensions";
  and a whole array on one side only, as `a() = 5`, "`a()` is a whole array, and the other side of `=` is not one".
  <!-- src: language/arrays-and-types "Whole-array assignment checks" -->

## Differences

From QB64pe: for arrays of a type with `_DYNAMIC` member arrays, with other bounds, it raises error 9 and still
copies, as many elements as the target has, reading past the source when it has fewer. QB64Rust copies nothing then
(`DIVERGENCES-QB64PE.md` D-022).
<!-- src: D-022; language/arrays-and-types "Whole-array assignment of owner types" -->

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
