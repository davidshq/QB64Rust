' QB 4.5 (beside SOMEDAY's row on LEN of an array element): does LEN of an element of a numeric or STRING * n
' array evaluate and check its index? QB64pe never does: LEN(a(9)) raises nothing and LEN(a(f(1))) does not
' call f. The FUNCTION counts its calls in ncall%, printed after each LEN (CALLS is a keyword in QB 4.5).
DECLARE FUNCTION f% (n AS INTEGER)
DIM SHARED ncall%
ON ERROR GOTO h
DIM a(3) AS INTEGER, s(3) AS STRING * 8, v$(3)
OPEN "OUT.TXT" FOR OUTPUT AS #1
ix = 9
k = LEN(a(ix))
PRINT #1, "1. LEN(a(ix)), ix = 9:"; k
ncall% = 0: k = LEN(a(f(1)))
PRINT #1, "2. LEN(a(f(1))):"; k; "calls"; ncall%
k = LEN(s(ix))
PRINT #1, "3. LEN(s(ix)), STRING * 8:"; k
ncall% = 0: k = LEN(s(f(2)))
PRINT #1, "4. LEN(s(f(2))):"; k; "calls"; ncall%
k = -1: k = LEN(v$(ix))
PRINT #1, "5. LEN(v$(ix)), variable-length:"; k
ncall% = 0: k = LEN(v$(f(1)))
PRINT #1, "6. LEN(v$(f(1))):"; k; "calls"; ncall%
CLOSE #1
SYSTEM
h:
PRINT #1, "   error"; ERR
RESUME NEXT

FUNCTION f% (n AS INTEGER)
    ncall% = ncall% + 1
    f% = n
END FUNCTION
