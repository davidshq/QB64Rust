$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Statement calls"): RESUME runs a failing statement call again; a raising
' argument is evaluated again too (the counter in the FUNCTION shows how often).
ON ERROR GOTO h
DIM SHARED calls AS LONG
DIM tries AS LONG
PRINT "KILL with RESUME"
KILL "v22_a_r.tmp"
PRINT "after KILL: handler ran"; tries
tries = 0
PRINT "RMDIR with RESUME, the name from a FUNCTION"
RMDIR dirname$
PRINT "after RMDIR: handler ran"; tries; "the FUNCTION"; calls
tries = 0: calls = 0
PRINT "KILL with a raising argument and RESUME NEXT"
KILL bad$
PRINT "after: handler ran"; tries; "the FUNCTION"; calls
SYSTEM
h:
tries = tries + 1
PRINT "  error"; ERR; "try"; tries
IF ERR = 53 AND tries = 1 THEN
    OPEN "v22_a_r.tmp" FOR OUTPUT AS #1: CLOSE #1
    RESUME
END IF
IF ERR = 76 AND tries = 1 THEN
    MKDIR "v22_a_rdir"
    RESUME
END IF
RESUME NEXT

FUNCTION dirname$
    calls = calls + 1
    dirname$ = "v22_a_rdir"
END FUNCTION

FUNCTION bad$
    calls = calls + 1
    bad$ = CHR$(-1)
END FUNCTION
