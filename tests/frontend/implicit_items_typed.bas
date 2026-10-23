' TEST: typed
$CONSOLE:ONLY
' Below $DYNAMIC, an array used without DIM first named by a part of a statement of items (m4-names-implicit-arrays
' task 4.1, design D1): its allocation runs just before that part, after the parts before it (measured,
' verification\v24_x264, x265; the old compiler's C++ for the file forms and PRINT USING, read in task 1.2).
DIM i AS LONG, a AS LONG, b AS LONG
'$DYNAMIC
PRINT " x"; dp(1); " y"
PRINT pa(1); " z", pz(1)
WRITE i, dr(1)
READ a, dq(1), b
READ rd(1), a
INPUT a, ia(1), b
INPUT "p"; ib(1), a
LINE INPUT ls$(1)
OPEN "x.txt" FOR INPUT AS #1
INPUT #1, a, fa(1), b
LINE INPUT #1, fl$(1)
PRINT #1, a; fp(1); b
WRITE #1, a, fw(1), b
PRINT USING "##"; a; fu(1)
PRINT USING "##"; fv(1)
PRINT "a"; USING fs$(1); a
SYSTEM
DATA 1, 2, 3, 4, 5
