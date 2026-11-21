' QB 4.5 (beside SOMEDAY's qb45-decl-errors row, form 3; QB64pe: verification\v24_x126): DIM a(3) without AS in a
' SUB after STATIC a() AS DOUBLE, called twice. QB64pe makes a SINGLE array that takes the plain name, sized anew on
' each call. Does QB 4.5 reject the DIM, size the DOUBLE array, or make another one?
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
CALL s
CALL s
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT

SUB s
    STATIC a() AS DOUBLE
    DIM a(3)
    a(1) = a(1) + 1
    PRINT #1, "sub:"; a(1); LEN(a(1)); UBOUND(a)
END SUB
