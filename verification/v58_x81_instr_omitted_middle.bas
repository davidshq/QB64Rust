$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x81_instr_omitted_middle)
' INSTR with its middle argument left out.
k = 0
IF k = 1 THEN
    x = INSTR(1, , "a")
END IF
PRINT "ok"
SYSTEM
