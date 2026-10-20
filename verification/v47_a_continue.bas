$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): _CONTINUE in each kind of loop.
' FOR, through a single-line IF
FOR i = 1 TO 4
    IF i = 2 THEN _CONTINUE
    PRINT i;
NEXT
PRINT
' FOR with a STEP, through a block IF and ELSE
FOR i = 10 TO 1 STEP -3
    IF i = 7 THEN
        _CONTINUE
    ELSE
        PRINT i;
    END IF
NEXT
PRINT "after"; i
' FOR, through SELECT CASE (as colors/color_with_dest)
FOR i = 1 TO 7
    SELECT CASE i
        CASE 3, 6: _CONTINUE
        CASE ELSE: PRINT i;
    END SELECT
NEXT
PRINT
' DO ... LOOP UNTIL: the test still runs
n = 0
DO
    n = n + 1
    IF n < 3 THEN _CONTINUE
    PRINT n;
LOOP UNTIL n >= 5
PRINT
' DO ... LOOP WHILE
n = 0
DO
    n = n + 1
    IF n MOD 2 = 0 THEN _CONTINUE
    PRINT n;
LOOP WHILE n < 7
PRINT
' DO WHILE ... LOOP
n = 0
DO WHILE n < 5
    n = n + 1
    IF n = 4 THEN _CONTINUE
    PRINT n;
LOOP
PRINT
' DO UNTIL ... LOOP
n = 0
DO UNTIL n >= 5
    n = n + 1
    IF n = 1 THEN _CONTINUE
    PRINT n;
LOOP
PRINT
' A plain DO ... LOOP left by EXIT DO
n = 0
DO
    n = n + 1
    IF n > 6 THEN EXIT DO
    IF n = 3 THEN _CONTINUE
    PRINT n;
LOOP
PRINT
' WHILE ... WEND
n = 0
WHILE n < 5
    n = n + 1
    IF n = 2 THEN _CONTINUE
    PRINT n;
WEND
PRINT
' The innermost loop: a WHILE in a FOR
FOR i = 1 TO 2
    n = 0
    WHILE n < 3
        n = n + 1
        IF n = 2 THEN _CONTINUE
        PRINT i * 10 + n;
    WEND
    PRINT "/";
NEXT
PRINT
' A FOR in a DO, with lower case and after a colon
n = 0
DO
    n = n + 1
    FOR j = 1 TO 3
        IF j = 2 THEN PRINT "-";: _continue
        PRINT n * 10 + j;
    NEXT
LOOP UNTIL n = 2
PRINT
' In a SUB and a FUNCTION
show
PRINT total(5)
SYSTEM

SUB show
    FOR k = 1 TO 5
        IF k MOD 2 = 0 THEN _CONTINUE
        PRINT k;
    NEXT
    PRINT
END SUB

FUNCTION total (m)
    t = 0
    FOR k = 1 TO m
        IF k = 3 THEN _CONTINUE
        t = t + k
    NEXT
    total = t
END FUNCTION
