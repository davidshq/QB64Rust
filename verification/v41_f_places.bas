$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): the places of an array: REDIMmed (also with _PRESERVE),
' $DYNAMIC, SHARED, a local and a STATIC array of a SUB, an array parameter (its number of dimensions not known
' where the SUB is), COMMON SHARED, OPTION BASE 1.
OPTION BASE 1
COMMON SHARED cs() AS LONG
DIM SHARED sh(5) AS LONG
DIM i AS LONG
REDIM cs(5) AS LONG
REDIM r(3) AS LONG
FOR i = 1 TO 3: r(i) = i: NEXT
_ARRAYCOPY r() TO sh(2)
PRINT "1. REDIM to SHARED:";: FOR i = 1 TO 5: PRINT sh(i);: NEXT: PRINT
REDIM _PRESERVE r(6) AS LONG
_ARRAYCOPY sh() TO r(2)
PRINT "2. after _PRESERVE:";: FOR i = 1 TO 6: PRINT r(i);: NEXT: PRINT
'$DYNAMIC
DIM dz(4) AS LONG
'$STATIC
_ARRAYCOPY dz() TO cs()
PRINT "3. $DYNAMIC to COMMON SHARED:";: FOR i = 1 TO 5: PRINT cs(i);: NEXT: PRINT
inside
PRINT "4. by a SUB on SHARED:";: FOR i = 1 TO 5: PRINT sh(i);: NEXT: PRINT
DIM p(1 TO 3) AS LONG, q(1 TO 5) AS LONG
p(1) = 7: p(2) = 8: p(3) = 9
param p(), q()
PRINT "5. parameters:";: FOR i = 1 TO 5: PRINT q(i);: NEXT: PRINT
DIM p2(1 TO 2, 1 TO 2) AS LONG, q2(1 TO 2, 1 TO 2) AS LONG
p2(1, 1) = 1: p2(2, 1) = 2: p2(1, 2) = 3: p2(2, 2) = 4
param2 p2(), q2()
PRINT "6. 2D parameters:"; q2(1, 1); q2(2, 1); q2(1, 2); q2(2, 2)
SYSTEM

SUB inside
    DIM lc(3) AS LONG
    STATIC st(3) AS LONG
    lc(1) = 31: lc(2) = 32: lc(3) = 33
    _ARRAYCOPY lc() TO st()
    _ARRAYCOPY st(1 TO 2) TO sh(4)
END SUB

SUB param (a() AS LONG, b() AS LONG)
    _ARRAYCOPY a() TO b(LBOUND(b) + 1)
END SUB

SUB param2 (a() AS LONG, b() AS LONG)
    _ARRAYCOPY a(1 TO 2, 1 TO 2) TO b(1, 1)
END SUB
