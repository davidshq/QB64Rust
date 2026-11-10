' QB 4.5 (beside SOMEDAY's row on an array without AS beside one listed by COMMON name() AS type, v66_c04): does
' DIM g(3) without AS size the COMMON array of DOUBLE, or make another array? QB64pe makes another one, so g(1)
' is the COMMON array, never sized, and raises error 9. g(2) = 1# / 3 shows the type by its digits.
COMMON SHARED g() AS DOUBLE
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM g(3)
PRINT #1, "1. g(1) = 1.5:";
g(1) = 1.5
PRINT #1, g(1)
PRINT #1, "2. g(2) = 1# / 3:";
g(2) = 1# / 3
PRINT #1, g(2)
PRINT #1, "3. UBOUND(g):";
PRINT #1, UBOUND(g)
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR
RESUME NEXT
