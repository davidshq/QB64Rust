# ABSOLUTE

`CALL ABSOLUTE(arguments, offset)` runs 16-bit machine code at an offset in the `DEF SEG` segment, through libqb's
emulator of the processor.

## Syntax

```text
CALL ABSOLUTE([argument, ...] offset)
```

## Behavior

A variable is passed as its offset, so the code can store into it: machine code that writes 1234 through its argument
leaves 1234 in `v%`.
<!-- src: language/conventional-memory "CALL ABSOLUTE"; verification\v61_c2_store_argument; tests\reference\examples\absolute_1 -->

Any other value is passed as the offset of a slot that holds it; a `SINGLE`, `DOUBLE` or `_INTEGER64` keeps its type.
<!-- src: language/conventional-memory "CALL ABSOLUTE"; verification\v61_c3_string_single -->

A program's own SUB named `ABSOLUTE` does not change `CALL ABSOLUTE(...)`.
<!-- src: language/conventional-memory "The name ABSOLUTE"; verification\v61_c9_absolute_sub -->

## Errors

- `ABSOLUTE(...)` without `CALL`, and `CALL ABSOLUTE` without parentheses, are compile errors.
  <!-- src: language/conventional-memory "The name ABSOLUTE"; verification\v61_c5_no_call_parens; verification\v61_c6_no_call; verification\v61_c7_call_no_parens -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:absolute_1 -->
```qb64
DIM code(15) AS INTEGER
DEF SEG = VARSEG(code(0))
FOR i = 0 TO 13
    READ b
    POKE VARPTR(code(0)) + i, b
NEXT
v% = 7
CALL ABSOLUTE(v%, VARPTR(code(0)))
DEF SEG
PRINT "The machine code stored 1234:"; v%
END
' push bp; mov bp,sp; mov bx,[bp+6]; mov word [bx],1234; pop bp; retf 2
DATA &H55,&H8B,&HEC,&H8B,&H5E,&H06,&HC7,&H07,&HD2,&H04,&H5D,&HCA,&H02,&H00
```

Output:

```text
The machine code stored 1234: 1234 

Press any key to continue
```
<!-- end generated -->

## See also

[`DEF SEG`](def-seg.md), [`VARPTR`](varptr.md), [`POKE`](poke.md), [`SADD`](sadd.md).
