$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): ABSOLUTE without CALL and without parentheses.
DIM code(1) AS INTEGER
code(0) = &HCB
DEF SEG = VARSEG(code(0))
ABSOLUTE VARPTR(code(0))
DEF SEG
PRINT "back"
SYSTEM
