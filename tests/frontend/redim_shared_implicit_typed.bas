' TEST: typed
$CONSOLE:ONLY
' In a procedure, REDIM of a shared array whose bound first names an array used without DIM (m4-left-marked task
' 3.1, design D2): the name is an array of the procedure in the whole procedure, above the REDIM too, made on each
' call (also in a SUB ... STATIC), and the shared array stays the main module's (verification\v55_x84, v60_b1-b10).
' In q2 the array used without DIM is named above the REDIM, so the REDIM gives the shared array its bounds (b7);
' in q3 only the item whose bound names the new array is the procedure's (b10).
REDIM SHARED sd(1), se(1) AS LONG, sf(1)
q1
q2
q3
SYSTEM

SUB q1
    PRINT UBOUND(sd)
    REDIM sd(lc(1) + 4)
    REDIM se(ld(1) + 5) AS LONG
    PRINT UBOUND(sd); se(1)
END SUB

SUB q2
    PRINT lm(1)
    REDIM sd(lm(1) + 4)
END SUB

SUB q3 STATIC
    REDIM sf(6), sd(ln(1) + 7)
    PRINT UBOUND(sf); UBOUND(sd)
END SUB
