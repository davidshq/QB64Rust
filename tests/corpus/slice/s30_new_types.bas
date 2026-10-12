$CONSOLE:ONLY
' Slice program (m2-numeric-types, task 5.4): the new numeric types' declarations (every AS spelling and suffix),
' literals (held as written, believed the suffix's type), LEN, stores, and the scenarios of the numeric-semantics
' spec delta that the old compiler agrees with (not the decided divergences D-005, D-006, D-009, nor the compile
' errors; HEX$ of a new type comes with task 8.4). No PRINT comma.
DIM a AS _BYTE, b AS _UNSIGNED _BYTE, c AS INTEGER, d AS _UNSIGNED INTEGER, e AS LONG, f AS _UNSIGNED LONG
DIM g AS _INTEGER64, h AS _UNSIGNED _INTEGER64, o AS _OFFSET, uo AS _UNSIGNED _OFFSET
DIM t AS _BIT, ut AS _UNSIGNED _BIT, t7 AS _BIT*7, ut9 AS _UNSIGNED _BIT * 9, s AS _UNSIGNED STRING
PRINT LEN(a); LEN(b); LEN(c); LEN(d); LEN(e); LEN(f); LEN(g); LEN(h); LEN(o); LEN(uo)
' Stores wrap to the width and signedness.
a = 200: b = 300: c = 70000: d = -1: e = 3000000000: f = -1: g = -1: h = -1: o = -1: uo = -1
PRINT a; b; c; d; e; f; g; h; o; uo
t = 1: ut = 3: t7 = 100: ut9 = 1000: s = "plain"
PRINT t; ut; t7; ut9; s
' Suffixes: one name, a variable per suffix.
n%% = 1: n~%% = 2: n~% = 3: n~& = 4: n~&& = 5: n%& = 6: n~%& = 7: n` = 1: n`5 = 9: n~` = 1: n~`5 = 33
PRINT n%%; n~%%; n~%; n~&; n~&&; n%&; n~%&; n`; n`5; n~`; n~`5
PRINT LEN(n%%); LEN(n~%&)
' Scenarios: numeric types.
DIM ub AS _UNSIGNED _BYTE: ub = 255: PRINT ub; LEN(ub)
x~% = 65535: PRINT x~%
' A pad before the _BIT * 64, which the old compiler would let overwrite the _BIT before it (D-009).
DIM pad AS _BIT * 32, u64 AS _UNSIGNED _BIT * 64: u64 = -1: PRINT u64
' Arithmetic with narrow operands.
b~%% = 255: PRINT b~%% + 1; x~% * 2
' _BIT stores: masked or sign-extended; in arithmetic a 32-bit storage.
DIM u3 AS _UNSIGNED _BIT * 3, b3 AS _BIT * 3, one AS _BIT
u3 = 7: PRINT u3 * 1000000000; u3 > -1
b3 = 5: PRINT b3
u3 = 13: PRINT u3
one = 1: PRINT one
' Literals.
PRINT -3; 40000; -2147483648; 4294967295~&; 255~%%; &HFF%%; &HFF%% + 0
PRINT 300~%%; -1~&; 40000%; 300~%% + 0; -1~& + 0
l& = 300~%%: PRINT l&
PRINT 4294967295~& + 1; -1~&& + 0; -1~&& > 0; 255~%% = -1%%
PRINT 1`; 9`3; 9`3 + 0
PRINT &H1FF~%%; &H1FF~%% + 0
' Stores from floats: half to even; 16 bits or fewer from SINGLE, wider and _BIT from _FLOAT.
x% = 2.5: PRINT x%;: x% = 3.5: PRINT x%
dd# = 2.5000001: x% = dd#: l& = dd#: PRINT x%; l&
DIM b16 AS _BIT * 16: u~% = dd#: b16 = dd#: PRINT u~%; b16
x% = 70000: PRINT x%
x~% = -1: u~&& = -1: PRINT x~%; u~&&
b%% = 200: PRINT b%%
' Logical operators and integer division (old types; unchanged).
PRINT 0.4 _ANDALSO 1; _NEGATE 0.4
PRINT 1.5 AND 3; 2.5 OR 0; NOT 1.5; 5 XOR 3; 5 EQV 3; 5 IMP 3
PRINT 1 _ANDALSO 2; 0 _ORELSE 0; _NEGATE 0; _NEGATE 5
PRINT 7 \ 2; -7 \ 2; 7.5 \ 2; -7 MOD 3; 7.5 MOD 2
' _OFFSET division keeps the type.
uo = 7: PRINT uo / 2
SYSTEM
