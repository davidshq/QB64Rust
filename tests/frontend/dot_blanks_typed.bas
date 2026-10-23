' TEST: typed
$CONSOLE:ONLY
' Blanks around the dot after a plain name (m4-devices-continue-paths task 3.1, design D2;
' verification\v19_dot_blanks_*, v47_c_dot_blanks): each line is read as the same line without the blanks, a member
' of a TYPE variable or a plain dotted name: read, assigned, an element of a member array, ERASE, LBOUND, a member
' passed by reference, nested members, a plain dotted scalar and array.
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
a . n = 3
PRINT a . n
a . s(2) = 7
PRINT a . s(2); LBOUND(a . s)
ERASE a . S
bump a . n
a . w . v = 9
PRINT a . w.v
p . q = 4
PRINT p . q
p . q . z = 5
p . r(1) = 6
PRINT LBOUND(p . r)
ERASE p . r
SUB bump (x AS LONG)
    x = x + 1
END SUB
