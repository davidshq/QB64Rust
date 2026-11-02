$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-implicit-arrays row): STATIC v(n), a bound not known when compiling, in a SUB beside a shared array v.
DIM SHARED v(3)
v(1) = 5
t
PRINT "main:"; v(1); UBOUND(v)
SYSTEM

SUB t
    n = 4
    STATIC v(n)
    PRINT "in t:"; v(1); UBOUND(v)
END SUB
