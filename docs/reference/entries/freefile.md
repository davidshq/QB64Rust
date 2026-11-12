# FREEFILE

`FREEFILE` gives a file number that no open file uses, for `OPEN … AS #f`.

## Syntax

<!-- begin generated: syntax -->
```text
_FREEFILE
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

An INTEGER.

## Behavior

`FREEFILE` gives the lowest number that is not open: 1 when no file is open, then 2 once 1 is.
<!-- src: language/file-io "File functions"; tests\reference\examples\freefile_1 -->

It is written without parentheses.
<!-- src: language/builtin-functions "Built-ins without parentheses" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:freefile_1 -->
```qb64
f = FREEFILE
OPEN "freefile_1.tmp" FOR OUTPUT AS #f
PRINT "The first free number was"; f; "and the next is"; FREEFILE
CLOSE #f
KILL "freefile_1.tmp"
END
```

Output:

```text
The first free number was 1 and the next is 2 

Press any key to continue
```
<!-- end generated -->

## See also

`OPEN`, `CLOSE`.
