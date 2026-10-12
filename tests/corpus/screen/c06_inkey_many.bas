' 600 characters and Enter: more than the 15 keys the INKEY$ buffer holds; none may be lost.
DO
    k$ = INKEY$
    IF LEN(k$) = 1 AND k$ <> CHR$(13) THEN
        n = n + 1
        sum& = sum& + ASC(k$) * (n MOD 7 + 1)
    END IF
    _LIMIT 100
LOOP UNTIL k$ = CHR$(13)
PRINT n; sum&
SYSTEM
