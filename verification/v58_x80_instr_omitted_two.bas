$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x80_instr_omitted_two)
' INSTR with its first argument left out and one more.
k = 0
IF k = 1 THEN
    x = INSTR(, "a")
END IF
PRINT "ok"
SYSTEM
