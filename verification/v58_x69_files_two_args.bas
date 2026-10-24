$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): does the old compiler accept this? (x69_files_two_args)
' FILES with two arguments.
k = 0
IF k = 1 THEN
    FILES "a", "b"
END IF
PRINT "ok"
SYSTEM
