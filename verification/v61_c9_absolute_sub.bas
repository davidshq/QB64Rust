$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a program's own SUB named ABSOLUTE: CALL ABSOLUTE(…) still runs the
' machine code (a far return), and the SUB is called without CALL.
DIM code(1) AS INTEGER
code(0) = &HCB
DEF SEG = VARSEG(code(0))
CALL ABSOLUTE(VARPTR(code(0)))
DEF SEG
PRINT "back"
ABSOLUTE 5
SYSTEM

SUB ABSOLUTE (n AS INTEGER)
    PRINT "mine"; n
END SUB
