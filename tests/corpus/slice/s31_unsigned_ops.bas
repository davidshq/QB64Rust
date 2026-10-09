$CONSOLE:ONLY
' Slice program (m2-numeric-types, task 5.3): the operator rules for the new integer types that the differential
' programs corrected, as readable examples (design D4; tasks.md 5.3 lists the corrections). Computed in the type C++
' gives the old compiler's expression, printed as the old compiler believes it. No PRINT comma.
DIM ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM b AS _BYTE, q AS _INTEGER64, o AS _OFFSET, uo AS _UNSIGNED _OFFSET
' The _BIT * 40 comes first: in the old compiler it overwrites the _BIT declared just before it (D-009).
DIM w AS _UNSIGNED _BIT * 40, u3 AS _UNSIGNED _BIT * 3, b3 AS _BIT * 3, u7 AS _UNSIGNED _BIT * 7
ub = 255: ui = 65535: ul = 4000000000: uq = 1: b = -1: q = 0
' Below 32 bits every operand is an int; at 32 bits unsigned wins; beside a 64-bit operand int64.
PRINT ub + 1; ui * 2; ub + b; ul * 2; ul + b; ul + q
' Compared in that type: -1 is 4294967295 as a uint32, but stays -1 beside an unsigned 16-bit value.
PRINT -1 < ul; -1 < ui; b < ul; b < ub
' Believed unsigned 64-bit only when both operands are unsigned and one is 64 bits wide.
PRINT ul + uq; ub * uq; uq - 2; ub - 256; -uq
ul = 4294967295
' EQV and IMP complement the left operand in its own width, then widen it.
PRINT ul EQV q; ul IMP q; q EQV ul; ub IMP q
' NOT of a narrow unsigned value is an int; of a 32-bit one unsigned.
PRINT NOT ub; NOT ul; NOT uq
' _ANDALSO and _ORELSE are believed as the other integer operators: two unsigned 64-bit operands print unsigned.
PRINT uq _ANDALSO uq; uq _ORELSE q; ub _ANDALSO ub
' A _BIT value counts as its 32- or 64-bit storage, believed by its width and signedness, printed through int64.
u3 = 7: b3 = -3: u7 = 100: w = 1099511627775
PRINT u3 * 1000000000; u3 > -1; b3 - u3; u7 + uq; w + uq; w * 16777216
' Literals wider than 32 bits get ll or ull: 0~`40 is a uint64.
PRINT 0~`40 > -1; -1~`40; -1~`40 + 0; 1`40 > -1
' _OFFSET: integers for + - and comparisons (a float operand rounded first), long double then qbr for * with a
' float and for /; believed _OFFSET, unsigned unless an _OFFSET operand is signed.
o = 7: uo = 7
PRINT o / 2; uo / 2; o * 1.5; uo * 2.5; o + 1.5; o - 2.5; o < 7.5; o > 6.5; o < 7.4
PRINT o - 8; uo - 8; uo - o - 1; o \ 2; o MOD 4; o AND 3
SYSTEM
