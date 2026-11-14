# READ

`READ` takes the next items of the program's `DATA` into variables: `READ name$, men`.

## Syntax

```text
READ variable [, variable]...
```

## Behavior

Each variable takes the next item: a number converted to the variable's type, a string its text.
<!-- src: language/data-read "READ"; tests\reference\examples\read_1 -->

A quoted item, or an unquoted one that is no number, read into a number raises error 2; a number outside the
variable's range raises error 6. The variable is then 0, and the item stays unread for the next `READ`.
<!-- src: language/data-read "READ" -->

## Errors

- Reading when no item is left raises run-time error 4.
  <!-- src: language/data-read "READ"; tests\reference\examples\read_1 -->
- A target that is no variable, or the name of the FUNCTION it stands in, is a compile error.
  <!-- src: language/data-read "READ" -->

## Differences

From QuickBASIC 4.5: a variable whose `READ` raises error 2 keeps its value there. In QB64Rust, as in QB64pe, it is
set to 0 (`DIVERGENCES-QB45.md` Q-031).
<!-- src: Q-031 -->

## Availability

<!-- begin generated: availability -->
Supported. All 25 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:read_1 -->
```qb64
READ a, b$, c
PRINT a; b$; c
ON ERROR GOTO handler
READ d
END

handler:
PRINT "No item left: error"; ERR
RESUME NEXT
DATA 1, two, 3
```

Output:

```text
 1 two 3 
No item left: error 4 

Press any key to continue
```
<!-- end generated -->

## See also

[`DATA`](data.md), [`RESTORE`](restore.md).
