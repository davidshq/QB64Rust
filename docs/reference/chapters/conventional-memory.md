# Conventional memory

QuickBASIC programs reach memory by segment and offset with `DEF SEG`, `PEEK`, `POKE`, `VARPTR` and `VARSEG`. QB64Rust
emulates that memory, so such a program finds its variables where QB64pe's does.

The pages of this area: [DEF SEG](../entries/def-seg.md), [PEEK](../entries/peek.md), [POKE](../entries/poke.md),
[VARPTR](../entries/varptr.md), [VARSEG](../entries/varseg.md), [VARPTR$](../entries/varptr-str.md),
[SADD](../entries/sadd.md), [BLOAD](../entries/bload.md), [BSAVE](../entries/bsave.md),
[ABSOLUTE](../entries/absolute.md), [INP](../entries/inp.md), [OUT](../entries/out.md), [WAIT](../entries/wait.md).

## What lives there

A variable or array that `VARPTR`, `VARSEG`, `SADD` or `VARPTR$` names anywhere in the program lives in libqb's
emulated conventional memory, in QB64pe's order and sizes. `PEEK` and `POKE` through its address reach its bytes.
<!-- src: language/conventional-memory "Variables in conventional memory"; language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s158_varptr_strings -->

A static array is placed in the order of its `DIM`, each rounded up to 16 bytes; a dynamic one at each allocation.
<!-- src: language/conventional-memory "Arrays in conventional memory"; tests\corpus\slice\s73_conventional_arrays -->

A procedure's variables are placed there too when their address is taken, and so is the argument of every call to a
parameter whose address is taken.
<!-- src: language/conventional-memory "A procedure's variables in conventional memory"; language/conventional-memory "Parameters whose address is taken" -->

## Ports and machine code

`INP`, `OUT` and `WAIT` reach the ports libqb emulates: the palette, the keyboard and the display status. `CALL
ABSOLUTE` runs 16-bit machine code through libqb's emulator.
<!-- src: language/conventional-memory "INP, OUT and WAIT"; language/conventional-memory "CALL ABSOLUTE"; tests\reference\examples\absolute_1 -->
