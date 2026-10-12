' Outline fixture of the language server's tests: every kind of symbol, folding range and jump.
' A run of three comment lines folds;
' this is the third.
$CONSOLE:ONLY
DECLARE SUB bump (n AS INTEGER)
TYPE point
    x AS INTEGER
    AS LONG y, z
END TYPE
DIM p AS point
top:
bump 3
total = twice%(4)
IF total > 2 THEN
    PRINT "big"
ELSE
    PRINT "small"
END IF
FOR i = 1 TO 3
    PRINT i
NEXT
DO
    i = i - 1
LOOP UNTIL i < 0
WHILE i < 2
    i = i + 1
WEND
SELECT CASE i
    CASE 1
        PRINT "one"
    CASE ELSE
        PRINT "other"
END SELECT
$IF WIN THEN
    PRINT "windows"
$ELSE
    PRINT "elsewhere"
$END IF
ON ERROR GOTO handler
GOTO again
again:
10 PRINT "ten"
GOSUB 10
END
handler:
RESUME NEXT

SUB bump (n AS INTEGER)
    again:
    n = n + 1
    IF n < 5 THEN GOTO again
    CALL bump(n)
END SUB

FUNCTION twice% (v AS INTEGER)
    twice% = v * 2
END FUNCTION
