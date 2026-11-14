# LOCK

`LOCK #n, first TO last` keeps other programs from using part of a shared file; `UNLOCK` with the same range frees it.

## Syntax

```text
LOCK [#]filenumber[, [first][ TO last]]
UNLOCK [#]filenumber[, [first][ TO last]]
```

## Behavior

`LOCK` and `UNLOCK` compile to QB64pe's calls, with the range in bytes for a binary file and in records for a random
one.
<!-- src: language/record-files "LOCK and UNLOCK"; tests\reference\examples\lock_1 -->

## Errors

- They raise error 52 for a file that is not open, 5 for a record of 0 or below, 75 for a range the wrong way round,
  and 70 for an `UNLOCK` that matches no `LOCK` or a second `LOCK` of the same bytes.
  <!-- src: language/record-files "LOCK and UNLOCK" -->

## Availability

<!-- begin generated: availability -->
Supported. All 21 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:lock_1 -->
```qb64
OPEN "lock_1.tmp" FOR BINARY AS #1
s$ = "abcdefgh"
PUT #1, , s$
LOCK #1, 1 TO 4
PRINT "Bytes 1 to 4 are locked."
UNLOCK #1, 1 TO 4
PRINT "and unlocked again."
CLOSE #1
KILL "lock_1.tmp"
END
```

Output:

```text
Bytes 1 to 4 are locked.
and unlocked again.

Press any key to continue
```
<!-- end generated -->

## See also

[`OPEN`](open.md).
