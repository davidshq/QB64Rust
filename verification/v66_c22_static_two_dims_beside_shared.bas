$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-implicit-arrays row): STATIC a(5, 5) in a SUB beside a shared array a of one dimension.
DIM SHARED a(3)
a(1) = 5
t
PRINT "main:"; a(1); UBOUND(a)
SYSTEM

SUB t
    STATIC a(5, 5)
    a(1, 1) = 2
    PRINT "in t:"; a(1, 1); UBOUND(a, 1); UBOUND(a, 2)
END SUB
