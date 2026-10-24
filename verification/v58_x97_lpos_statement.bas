$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x97_lpos_statement)
' LPOS as a statement.
k = 0
IF k = 1 THEN
    LPOS 1
END IF
PRINT "ok"
SYSTEM
