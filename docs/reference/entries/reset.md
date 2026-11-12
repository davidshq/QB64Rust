# RESET

`RESET` closes every open file, as `CLOSE` with no file number does.

## Syntax

```text
RESET
```

## Behavior

`RESET` closes every open file: afterwards `FREEFILE` is 1 again.
<!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s03_reset; tests\reference\examples\reset_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:reset_1 -->
```qb64
OPEN "reset_1.tmp" FOR OUTPUT AS #1
OPEN "reset_1b.tmp" FOR OUTPUT AS #2
RESET
PRINT "After RESET, FREEFILE is"; FREEFILE
KILL "reset_1.tmp": KILL "reset_1b.tmp"
END
```

Output:

```text
After RESET, FREEFILE is 1 

Press any key to continue
```
<!-- end generated -->

## See also

`CLOSE`.
