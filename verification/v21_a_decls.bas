$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.1): every AS spelling of the new numeric types. Each is printed
' at the start, after storing -1, its smallest value, its largest value and one past the largest (literals of
' the smallest type that holds them), and LEN of each that LEN accepts (_BIT: a compile error, v21_x).
ON ERROR GOTO h
DIM sb AS _BYTE, ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM so AS _OFFSET, uo AS _UNSIGNED _OFFSET
DIM b1 AS _BIT, ub1 AS _UNSIGNED _BIT, b7 AS _BIT * 7, ub7 AS _UNSIGNED _BIT*7
DIM b32 AS _BIT * 32, ub32 AS _UNSIGNED _BIT * 32, b33 AS _BIT * 33, ub33 AS _UNSIGNED _BIT * 33
DIM b64 AS _BIT * 64, ub64 AS _UNSIGNED _BIT * 64
PRINT "start:"; sb; ub; ui; ul; uq; so; uo
PRINT "start bit:"; b1; ub1; b7; ub7; b32; ub32; b33; ub33; b64; ub64
sb = -1: ub = -1: ui = -1: ul = -1: uq = -1: so = -1: uo = -1
PRINT "-1:"; sb; ub; ui; ul; uq; so; uo
' Each _BIT * n above 32 is printed right after its store: such scalars overlap (D-009, v04_accidental).
b1 = -1: ub1 = -1: b7 = -1: ub7 = -1: b32 = -1: ub32 = -1
PRINT "-1 bit:"; b1; ub1; b7; ub7; b32; ub32
b33 = -1: PRINT "-1 _BIT * 33:"; b33
ub33 = -1: PRINT "-1 _UNSIGNED _BIT * 33:"; ub33
b64 = -1: PRINT "-1 _BIT * 64:"; b64
ub64 = -1: PRINT "-1 _UNSIGNED _BIT * 64:"; ub64
sb = -128: PRINT "_BYTE -128:"; sb
sb = 127: PRINT "_BYTE 127:"; sb
sb = 128: PRINT "_BYTE 128:"; sb
ub = 255: PRINT "_UNSIGNED _BYTE 255:"; ub
ub = 256: PRINT "_UNSIGNED _BYTE 256:"; ub
ui = 65535: PRINT "_UNSIGNED INTEGER 65535:"; ui
ui = 65536: PRINT "_UNSIGNED INTEGER 65536:"; ui
ul = 4294967295: PRINT "_UNSIGNED LONG 4294967295:"; ul
ul = 4294967296: PRINT "_UNSIGNED LONG 4294967296:"; ul
uq = 9223372036854775807: PRINT "_UNSIGNED _INTEGER64 9223372036854775807:"; uq
uq = 18446744073709551615~&&: PRINT "_UNSIGNED _INTEGER64 18446744073709551615~&&:"; uq
so = -9223372036854775808: PRINT "_OFFSET -9223372036854775808:"; so
so = 9223372036854775807: PRINT "_OFFSET 9223372036854775807:"; so
uo = 18446744073709551615~&&: PRINT "_UNSIGNED _OFFSET 18446744073709551615~&&:"; uo
b1 = 1: PRINT "_BIT 1:"; b1
b1 = 2: PRINT "_BIT 2:"; b1
ub1 = 1: PRINT "_UNSIGNED _BIT 1:"; ub1
ub1 = 2: PRINT "_UNSIGNED _BIT 2:"; ub1
b7 = 63: PRINT "_BIT * 7 63:"; b7
b7 = 64: PRINT "_BIT * 7 64:"; b7
b7 = -64: PRINT "_BIT * 7 -64:"; b7
b7 = -65: PRINT "_BIT * 7 -65:"; b7
ub7 = 127: PRINT "_UNSIGNED _BIT * 7 127:"; ub7
ub7 = 128: PRINT "_UNSIGNED _BIT * 7 128:"; ub7
b32 = 2147483647: PRINT "_BIT * 32 2147483647:"; b32
b32 = 2147483648: PRINT "_BIT * 32 2147483648:"; b32
ub32 = 4294967295: PRINT "_UNSIGNED _BIT * 32 4294967295:"; ub32
ub32 = 4294967296: PRINT "_UNSIGNED _BIT * 32 4294967296:"; ub32
b33 = 4294967295: PRINT "_BIT * 33 4294967295:"; b33
b33 = 4294967296: PRINT "_BIT * 33 4294967296:"; b33
ub33 = 8589934591: PRINT "_UNSIGNED _BIT * 33 8589934591:"; ub33
ub33 = 8589934592: PRINT "_UNSIGNED _BIT * 33 8589934592:"; ub33
b64 = 9223372036854775807: PRINT "_BIT * 64 9223372036854775807:"; b64
b64 = -9223372036854775808: PRINT "_BIT * 64 -9223372036854775808:"; b64
ub64 = 18446744073709551615~&&: PRINT "_UNSIGNED _BIT * 64 18446744073709551615~&&:"; ub64
PRINT "LEN:"; LEN(sb); LEN(ub); LEN(ui); LEN(ul); LEN(uq); LEN(so); LEN(uo)
DIM us AS _UNSIGNED STRING
us = "abc"
PRINT "_UNSIGNED STRING: ["; us; "]"; LEN(us)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
