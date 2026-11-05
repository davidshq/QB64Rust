$CONSOLE:ONLY
' Verification (2026-11-05, found while checking the value trace of _FLOAT arrays): _ARRAYCOPY of _FLOAT member
' arrays. The elements of a member array lie 32 bytes apart and those of an array variable 16, and the copy
' takes one element size for both sides: member to member, and the two copies between a variable and a member.
TYPE t
    v(3) AS _FLOAT
END TYPE
DIM r AS t, r2 AS t, a(3) AS _FLOAT, b(3) AS _FLOAT
a(0) = 10.5: a(1) = 11.5: a(2) = 12.5: a(3) = 13.5
_ARRAYCOPY a() TO b()
PRINT "variable to variable:"; b(0); b(1); b(2); b(3)
_ARRAYCOPY a() TO r.v()
PRINT "variable to member:"; r.v(0); r.v(1); r.v(2); r.v(3)
_ARRAYCOPY r.v() TO b()
PRINT "member to variable:"; b(0); b(1); b(2); b(3)
r2.v(0) = 1.5: r2.v(1) = 2.5: r2.v(2) = 3.5: r2.v(3) = 4.5
_ARRAYCOPY r2.v() TO r.v()
PRINT "member to member:"; r.v(0); r.v(1); r.v(2); r.v(3)
_ARRAYCOPY r2.v(1 TO 2) TO r.v(2)
PRINT "member range to member:"; r.v(0); r.v(1); r.v(2); r.v(3)
DIM c(3) AS _FLOAT
_ARRAYCOPY r2.v() TO c()
PRINT "member to an empty variable:"; c(0); c(1); c(2); c(3)
PRINT "LEN:"; LEN(a(0)); LEN(r)
SYSTEM
