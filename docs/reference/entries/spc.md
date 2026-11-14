# SPC

`SPC(n)` in a `PRINT` prints `n` blanks. As a value it is a string of `n` spaces.

## Syntax

<!-- begin generated: syntax -->
```text
SPC(count%)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `count%`: the number of blanks.

## Result

A string of `n` spaces.

## Behavior

`PRINT "a"; SPC(5); "b"` prints five blanks between `a` and `b`.
<!-- src: language/screen-output "TAB and SPC in text output"; tests\reference\examples\spc_1 -->

`SPC(n)` is also a value: `t$ = SPC(3)` holds three spaces.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\spc_1 -->

## Errors

- A string as the count is a compile error.
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. All 12 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:spc_1 -->
```qb64
PRINT "a"; SPC(5); "b"
t$ = SPC(3)
PRINT "["; t$; "]"; LEN(t$)
END
```

Output:

```text
a     b
[   ] 3 

Press any key to continue
```
<!-- end generated -->

## See also

[`TAB`](tab.md), [`SPACE$`](space-str.md), [`PRINT`](print.md).
