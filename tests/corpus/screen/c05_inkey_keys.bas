' Keys from the keys file: characters, special keys between them, Enter last.
DO
    k$ = INKEY$
    IF LEN(k$) = 1 THEN PRINT ASC(k$);
    IF LEN(k$) = 2 THEN PRINT "x"; ASC(k$, 2);
    _LIMIT 200
LOOP UNTIL k$ = CHR$(13)
SYSTEM
