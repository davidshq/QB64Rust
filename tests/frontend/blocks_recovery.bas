' TEST: check-fail
$CONSOLE:ONLY
' Block recovery (m2-parser-breadth design D4, task 6.1): one error per mistake, and the statements around it still
' parse and are checked (the strings stored in numbers below). Every case is rejected by the old compiler
' (verification\v16_m4_*); the messages are new.
FOR i = 1 TO 2
    IF i THEN
NEXT
END IF
x = "a"
WEND
FOR k = 1 TO 2: IF k THEN NEXT
NEXT k
IF k THEN FOR m = 1 TO 2: PRINT m
SELECT CASE k
    PRINT 1
    CASE 1
        IF k THEN
    CASE 2
        END IF
END SELECT
TYPE t
    a AS LONG
    y = "b"
END TYPE
DECLARE LIBRARY
    z = "c"
END DECLARE
FOR j = 1 TO 2: NEXT j, i
EXIT DO
DO WHILE 0
LOOP UNTIL 1
IF 1 THEN
ELSE
ELSE
END IF
IF k THEN PRINT 1 ELSE PRINT 2 ELSE PRINT 3
SUB s
    FOR n = 1 TO 2
END SUB
WHILE 1
SUB u
END SUB
DO
