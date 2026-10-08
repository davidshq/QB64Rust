$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.2): stores into _BIT variables: the mask (unsigned), the sign
' extension (signed), floats rounded into each width (16 bits and below from SINGLE, 17 to 31 from DOUBLE,
' 32 and above from _FLOAT: study\02 section 1.5), wide integers, and _BIT values in arithmetic. No two
' _BIT * n above 32 are alive in one line (they overlap, D-009).
ON ERROR GOTO h
DIM b1 AS _BIT, u1 AS _UNSIGNED _BIT, b3 AS _BIT * 3, u3 AS _UNSIGNED _BIT * 3
b3 = 5: PRINT "b3 5:"; b3
b3 = 4: PRINT "b3 4:"; b3
b3 = 3: PRINT "b3 3:"; b3
b3 = 8: PRINT "b3 8:"; b3
b3 = -5: PRINT "b3 -5:"; b3
b3 = 13: PRINT "b3 13:"; b3
b3 = -4: PRINT "b3 -4:"; b3
u3 = 13: PRINT "u3 13:"; u3
u3 = -1: PRINT "u3 -1:"; u3
u3 = 8: PRINT "u3 8:"; u3
b1 = 1: PRINT "b1 1:"; b1
b1 = 2: PRINT "b1 2:"; b1
b1 = 3: PRINT "b1 3:"; b1
b1 = -2: PRINT "b1 -2:"; b1
u1 = 3: PRINT "u1 3:"; u1
u1 = -1: PRINT "u1 -1:"; u1
b3 = 2.5: PRINT "b3 2.5:"; b3
b3 = 3.5: PRINT "b3 3.5:"; b3
b3 = -0.5: PRINT "b3 -0.5:"; b3
b3 = -1.5: PRINT "b3 -1.5:"; b3
u3 = 6.5: PRINT "u3 6.5:"; u3
u3 = 7.5: PRINT "u3 7.5:"; u3
DIM d AS DOUBLE, fl AS _FLOAT, q AS _INTEGER64
d = 2.5000001
DIM b16 AS _BIT * 16, b17 AS _BIT * 17, b31 AS _BIT * 31, b32 AS _BIT * 32
b16 = d: b17 = d: b31 = d: b32 = d
PRINT "2.5000001# into 16 17 31 32:"; b16; b17; b31; b32
d = 16777217.5#
b17 = d: b31 = d: b32 = d
PRINT "16777217.5# into 17 31 32:"; b17; b31; b32
d = 65537.5#
b17 = d: PRINT "65537.5# into 17:"; b17
DIM u20 AS _UNSIGNED _BIT * 20
u20 = -1.5: PRINT "u20 -1.5:"; u20
u20 = 1048575.5: PRINT "u20 1048575.5:"; u20
DIM b40 AS _BIT * 40
b40 = 1E+12: PRINT "b40 1E+12:"; b40
fl = 549755813887.5##
b40 = fl: PRINT "b40 549755813887.5##:"; b40
DIM u40 AS _UNSIGNED _BIT * 40
u40 = -2.5: PRINT "u40 -2.5:"; u40
DIM u64 AS _UNSIGNED _BIT * 64
u64 = 1.8E+19: PRINT "u64 1.8E+19:"; u64
DIM b7 AS _BIT * 7
q = 4294967298
b7 = q: PRINT "b7 4294967298&&:"; b7
q = -129
b7 = q: PRINT "b7 -129&&:"; b7
DIM ul AS _UNSIGNED LONG
ul = 4294967295
b7 = ul: PRINT "b7 4294967295~&:"; b7
b3 = -3: u3 = 7
PRINT "arith:"; b3 + 1; b3 * b3; u3 + 1; u3 * 1000000000; b3 - u3; -b3; NOT u3; b3 = -3; u3 > b3
DIM ub32 AS _UNSIGNED _BIT * 32
ub32 = 4294967295
PRINT "ub32 + 1:"; ub32 + 1; ub32 * 2; ub32 > 0; ub32 > -1
PRINT "HEX$:"; HEX$(b3); " "; HEX$(u3); " "; HEX$(b1); " "; HEX$(ub32)
PRINT "STR$: ["; STR$(b3); "] ["; STR$(u3); "]"
u64 = 18446744073709551615~&&
PRINT "u64 max:"; u64; u64 + 0; u64 > 0
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
