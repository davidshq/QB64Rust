$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a FUNCTION and a colon: a label, with and without a jump to it.
DIM n AS INTEGER
fone:
n = n + 1
IF n < 3 THEN GOTO fone
ftwo:
PRINT "n ="; n; fone; ftwo
SYSTEM
FUNCTION fone
    fone = 7
END FUNCTION
FUNCTION ftwo
    ftwo = 8
END FUNCTION
