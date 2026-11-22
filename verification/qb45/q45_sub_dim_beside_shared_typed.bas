' QB 4.5 (beside SOMEDAY's qb45-decl-errors row, form 3; QB64pe: verification\v48_x20): DIM c(3) without AS in a
' SUB, when the main module has DIM SHARED c(2) AS DOUBLE. QB64pe makes the SUB a SINGLE array of its own that takes
' the plain name. Does QB 4.5 reject the DIM, size the shared array, or make a local one?
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM SHARED c(2) AS DOUBLE
c(1) = 1# / 3
CALL s
PRINT #1, "main:"; c(1); UBOUND(c)
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT

SUB s
    DIM c(3)
    c(1) = c(1) + 1
    PRINT #1, "sub:"; c(1); LEN(c(1)); UBOUND(c)
END SUB
