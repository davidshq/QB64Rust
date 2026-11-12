# RESTORE

`RESTORE` makes the next `READ` start again at the first `DATA` item, and `RESTORE label` at the first `DATA` after a
label.

## Syntax

```text
RESTORE [label]
```

## Behavior

`RESTORE` alone goes back to the program's first item. `RESTORE label` goes to the first item of the first `DATA`
after that label in the file.
<!-- src: language/data-read "RESTORE"; tests\reference\examples\restore_1 -->

The label may stand in the main module or any procedure, whatever body the `RESTORE` is in. A line number may stand
where a label does.
<!-- src: language/data-read "RESTORE" -->

When no `DATA` follows the label, the next `READ` raises error 4.
<!-- src: language/data-read "RESTORE" -->

## Errors

- A label that does not exist, or that more than one body has, is a compile error.
  <!-- src: language/data-read "RESTORE" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:restore_1 -->
```qb64
READ x, y: PRINT x; y
RESTORE
READ x: PRINT x
RESTORE second
READ x: PRINT x
END
DATA 1, 2
second:
DATA 10, 20
```

Output:

```text
 1  2 
 1 
 10 

Press any key to continue
```
<!-- end generated -->

## See also

[`READ`](read.md), [`DATA`](data.md).
