' TEST: ir
$CONSOLE:ONLY
' In a procedure, REDIM of a shared array whose bound first names an array used without DIM (m4-left-marked task
' 3.1, design D2): the procedure's array of the name is made on each call and released at its end, and the REDIM
' and the uses above and below it name it; the main module's sd is untouched.
REDIM SHARED sd(1)
q
PRINT UBOUND(sd)
SYSTEM

SUB q
    PRINT UBOUND(sd)
    REDIM _PRESERVE sd(lc(1) + 4)
    PRINT UBOUND(sd)
END SUB
