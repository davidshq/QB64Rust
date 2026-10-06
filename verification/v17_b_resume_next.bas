$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): RESUME NEXT after an error in each kind of block header. The handler
' makes the failing call succeed from then on (k = 65); every loop body is capped at 6 passes.
DIM k AS LONG
ON ERROR GOTO h
PRINT "IF block"
k = -1
IF CHR$(k) = "A" THEN
    PRINT "  then"
ELSE
    PRINT "  else"
END IF
PRINT "WHILE"
k = -1: n = 0
WHILE CHR$(k) = "A" AND n < 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT WHILE
WEND
PRINT "DO UNTIL"
k = -1: n = 0
DO UNTIL CHR$(k) <> "A" OR n >= 2
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP
PRINT "LOOP WHILE"
k = -1: n = 0
DO
    n = n + 1: PRINT "  body"; n
    IF n > 6 THEN EXIT DO
LOOP WHILE CHR$(k) = "A" AND n < 3
PRINT "  after"; n
PRINT "FOR start"
k = -1: n = 0: i = 99
FOR i = ASC(CHR$(k)) - 63 TO 4
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "FOR step"
k = -1: n = 0: i = 99
FOR i = 1 TO 5 STEP ASC(CHR$(k)) - 63
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "FOR step, variable already 3"
k = -1: n = 0: i = 3
FOR i = 1 TO 5 STEP ASC(CHR$(k)) - 63
    n = n + 1: PRINT "  body"; i
    IF n > 6 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "FOR in a SUB"
k = -1
inner
PRINT "end"
SYSTEM

h:
errs = errs + 1
PRINT "  handler"; ERR
IF errs > 30 THEN PRINT "too many errors": SYSTEM
k = 65
RESUME NEXT

SUB inner
    SHARED k AS LONG
    n = 0
    FOR j = 1 TO LEN(CHR$(k)) + 2
        n = n + 1: PRINT "  body"; j
        IF n > 6 THEN EXIT FOR
    NEXT
    PRINT "  after"; j
END SUB
