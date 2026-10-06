$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): jumps into and out of blocks, EXIT through nested blocks. Every loop
' body is capped.
PRINT "GOTO into an IF body whose condition is false"
x = 0
GOTO inif
IF x = 1 THEN
    PRINT "  then 1"
    inif:
    PRINT "  then 2"
ELSE
    PRINT "  else"
END IF
PRINT "  after END IF"

PRINT "GOTO into a FOR body from outside (first FOR in the program)"
n = 0
GOTO infor
FOR i = 1 TO 3
    PRINT "  body 1"; i
    infor:
    n = n + 1: PRINT "  body 2"; i
    IF n > 6 THEN PRINT "  capped": EXIT FOR
NEXT
PRINT "  after"; i

PRINT "the same FOR entered normally, then GOTO into it again"
n = 0
FOR q = 1 TO 2
    n = n + 1: PRINT "  loop A"; q
    again:
    IF n > 6 THEN PRINT "  capped": EXIT FOR
NEXT
PRINT "  after A"; q
IF n < 3 THEN n = 5: GOTO again
PRINT "  after second"; q

PRINT "GOTO out of a FOR, then the FOR again"
FOR i = 1 TO 5
    IF i = 2 THEN GOTO outfor
    PRINT "  body"; i
NEXT
outfor:
PRINT "  out at"; i
FOR i = 1 TO 2
    PRINT "  second run"; i
NEXT

PRINT "GOTO into a WHILE body"
n = 0
GOTO inwhile
WHILE n < 3
    PRINT "  top"; n
    inwhile:
    n = n + 1: PRINT "  in"; n
    IF n > 6 THEN EXIT WHILE
WEND
PRINT "  after"; n

PRINT "GOTO into a DO ... LOOP UNTIL body"
n = 0
GOTO indo
DO
    PRINT "  top"; n
    indo:
    n = n + 1: PRINT "  in"; n
    IF n > 6 THEN EXIT DO
LOOP UNTIL n >= 3
PRINT "  after"; n

PRINT "EXIT FOR inside a WHILE inside a FOR"
FOR i = 1 TO 3
    w = 0
    WHILE w < 5
        w = w + 1
        IF i = 2 AND w = 2 THEN EXIT FOR
    WEND
    PRINT "  i"; i; "w"; w
NEXT
PRINT "  after"; i; w

PRINT "EXIT DO inside a FOR inside a DO, EXIT WHILE inside an IF"
n = 0
DO
    n = n + 1
    FOR j = 1 TO 3
        IF j = 2 THEN EXIT DO
    NEXT
LOOP
PRINT "  after DO"; n; j
w = 0
WHILE 1
    w = w + 1
    IF w = 4 THEN
        EXIT WHILE
    END IF
WEND
PRINT "  after WHILE"; w

PRINT "NEXT j, i and NEXT with suffix"
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT "  "; i; j
NEXT j, i!
DIM m AS LONG
FOR m = 1 TO 2
NEXT m&
PRINT "  m&"; m
SYSTEM
