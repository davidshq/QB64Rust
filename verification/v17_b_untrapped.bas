$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): untrapped errors in block headers, run with QB64PE_NOPROMPT=continue
' (v17_b_untrapped.noprompt), so the runtime reports each error and goes on. Every loop body is capped at 6 passes.
DIM k AS LONG
k = -1
PRINT "IF"
IF CHR$(k) = "A" THEN PRINT "  then" ELSE PRINT "  else"
PRINT "WHILE"
n = 0
WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT WHILE
WEND
PRINT "LOOP UNTIL"
n = 0
DO
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP UNTIL CHR$(k) = "A" OR n >= 3
PRINT "FOR limit"
n = 0
FOR i = 1 TO LEN(CHR$(k)) + 2
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "end"
SYSTEM
