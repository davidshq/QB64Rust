' TEST: typed
$CONSOLE:ONLY
' The typing of each operator family with operands of mixed signedness (m2-numeric-types design D4, task 5.1): the
' held type follows C's usual arithmetic conversions on the C types, the believed type the old compiler's markup
' (`_UNSIGNED _INTEGER64` only when both are unsigned and one is 64 bits wide, a `_BIT` by its width); `EQV` and
' `IMP` complement the left operand in its own width; an `_OFFSET` operand takes integers, except `*` with a float
' and `/`, which round a `long double` result; the result is believed `_OFFSET`. Same output as qb64pe.exe.
DIM l AS LONG, ul AS _UNSIGNED LONG, ub AS _UNSIGNED _BYTE, uq AS _UNSIGNED _INTEGER64, q AS _INTEGER64
DIM o AS _OFFSET, uo AS _UNSIGNED _OFFSET, u7 AS _UNSIGNED _BIT * 7, b3 AS _BIT * 3, s AS SINGLE
l = -1: ul = 4000000000: ub = 200: uq = 18446744073709551615~&&: q = -2
o = -7: uo = 7: u7 = 100: b3 = -3: s = 2.5
PRINT l < ul; ub + ub; ul * 2; ul + l
PRINT uq + u7; uq AND s; uq _ANDALSO uq; ul EQV q; ub IMP q
PRINT -uq; NOT ub; ub ^ 2; b3 - u7
PRINT o + s; uo * s; o / 2; uo - o; o < s
SYSTEM
