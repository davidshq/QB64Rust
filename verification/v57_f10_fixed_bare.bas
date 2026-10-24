$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a program FUNCTION fs$5 as a statement: fs$5 "a".
fs$5 "a"
PRINT "ok"
SYSTEM
FUNCTION fs$5 (a AS STRING)
    PRINT "in fs"
    fs$5 = "x"
END FUNCTION
