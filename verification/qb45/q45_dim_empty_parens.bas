' QB 4.5 (beside SOMEDAY's row on DIM a() and REDIM a()): what DIM a() with empty parentheses declares. QB64pe
' declares the scalar a and no array, so a(1) after it is another array, used without DIM.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM a()
a = 4
PRINT #1, "1. DIM a(), then a = 4:"; a
REDIM a(5)
a(3) = 7
PRINT #1, "2. REDIM a(5), a(3) = 7:"; a(3); UBOUND(a); a
DIM b()
b(2) = 9
PRINT #1, "3. DIM b(), b(2) = 9:"; b(2); UBOUND(b)
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT
