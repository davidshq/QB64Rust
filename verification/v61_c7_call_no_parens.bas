$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): CALL ABSOLUTE without parentheses.
DIM code(1) AS INTEGER
code(0) = &HCB
DEF SEG = VARSEG(code(0))
CALL ABSOLUTE VARPTR(code(0))
DEF SEG
PRINT "back"
SYSTEM
