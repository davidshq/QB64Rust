$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 6): in a SUB ... STATIC, DIM of the name of a
' DIM SHARED variable of the main module, typed with AS there. Does the DIM make the procedure's own variable
' (as in any other SUB), and is it kept between calls?
DIM SHARED w AS LONG
DIM SHARED v
w = 5: v = 6
s: s
PRINT "main:"; w; v
SYSTEM

SUB s STATIC
    DIM w
    DIM v
    w = w + 1.5: v = v + 2.5
    PRINT "SUB:"; w; v
END SUB
