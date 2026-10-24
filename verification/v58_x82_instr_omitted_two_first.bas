$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x82_instr_omitted_two_first)
' INSTR with its first two arguments left out.
k = 0
IF k = 1 THEN
    x = INSTR(, , "a")
END IF
PRINT "ok"
SYSTEM
