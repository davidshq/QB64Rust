$CONSOLE:ONLY
' Blanks around the dot after a plain name (m4-devices-continue-paths, design D2; verification\v19_dot_blanks_*,
' v47_c_dot_blanks): read as the name without them, a member of a TYPE variable or a plain dotted name; ERASE of a
' member array written so (as upstream arrays/04_float_family), its bounds, an element, a member passed by
' reference, nested members, a plain dotted array.
TYPE inner
    v AS LONG
END TYPE
TYPE t
    s(1 TO 3) AS LONG
    n AS LONG
    w AS inner
END TYPE
DIM a AS t
DIM p.r(2) AS LONG
a . s(2) = 7
PRINT a.s(2); a . s(2); LBOUND(a . s); UBOUND(a . s)
ERASE a . S
PRINT a.s(2)
a . n = 3
bump a . n
PRINT a . n
a . w . v = 9
PRINT a.w.v; a . w.v
p . q = 4
PRINT p.q; p . q
p . r(1) = 6
PRINT p.r(1); UBOUND(p . r)
ERASE p . r
PRINT p.r(1)
SYSTEM

SUB bump (x AS LONG)
    x = x + 1
END SUB
