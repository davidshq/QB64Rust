$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): blanks around the dot after a plain name.
TYPE inner
    v AS LONG
END TYPE
TYPE t
    s(1 TO 3) AS LONG
    n AS LONG
    w AS inner
END TYPE
DIM a AS t
a . s(2) = 7
PRINT a.s(2); a . s(2)
PRINT LBOUND(a . s); UBOUND(a . s)
a . n = 3
bump a . n
PRINT a.n; a . n
a . w . v = 9
PRINT a.w.v; a . w.v; a.w . v
' plain dotted names, no TYPE variable of that name
p.q = 4
PRINT p . q
p . q . r = 5
PRINT p.q.r
SYSTEM

SUB bump (x AS LONG)
    x = x + 1
END SUB
