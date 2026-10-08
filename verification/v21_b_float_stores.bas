$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, tasks 1.2 and 1.4): a DOUBLE stored into each integer type, to see which
' rounding each target uses: 2.5000001# rounds to 2 from SINGLE and to 3 from DOUBLE or _FLOAT; 16777217.5#
' is 16777216 from SINGLE and 16777218 otherwise. Also values beyond _INTEGER64 into the unsigned 64-bit types.
ON ERROR GOTO h
DIM d AS DOUBLE
DIM sb AS _BYTE, ub AS _UNSIGNED _BYTE, si AS INTEGER, ui AS _UNSIGNED INTEGER
DIM sl AS LONG, ul AS _UNSIGNED LONG, sq AS _INTEGER64, uq AS _UNSIGNED _INTEGER64
DIM so AS _OFFSET, uo AS _UNSIGNED _OFFSET
d = 2.5000001#
sb = d: ub = d: si = d: ui = d: sl = d: ul = d: sq = d: uq = d: so = d: uo = d
PRINT "2.5000001#:"; sb; ub; si; ui; sl; ul; sq; uq; so; uo
d = 16777217.5#
ui = d: sl = d: ul = d: sq = d: uq = d: so = d: uo = d
PRINT "16777217.5# (ui wraps):"; ui; sl; ul; sq; uq; so; uo
d = 1.8E+19
uq = d: uo = d: sq = d: so = d
PRINT "1.8D+19:"; uq; uo; sq; so
d = -1.5
ub = d: ui = d: ul = d: uq = d: uo = d
PRINT "-1.5:"; ub; ui; ul; uq; uo
d = 4294967295.5#
ul = d: PRINT "4294967295.5# into ~&:"; ul
DIM f AS SINGLE
f = 2.5
ub = f: ui = f: ul = f: uq = f
PRINT "2.5!:"; ub; ui; ul; uq
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
