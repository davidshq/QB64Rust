$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x92_mkdmbf_two_args)
' MKDMBF$ with two arguments.
k = 0
IF k = 1 THEN
    s$ = MKDMBF$(1, 2)
END IF
PRINT "ok"
SYSTEM
