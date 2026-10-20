' TEST: coverage
' A coverage build: one counter per source statement. The single-line IF's condition, THEN parts and ELSE part are
' four entries; its jump past the ELSE part gets none. Each CASE is one entry; the jump to END SELECT gets none.
' A one-line loop's bottom (the FOR's step at NEXT, the test at LOOP, the jump at WEND) is an entry apart from
' its header's.
$CONSOLE:ONLY
a = 1
IF a = 2 THEN PRINT "x": b = 2 ELSE PRINT "y"
SELECT CASE a
    CASE 1
        PRINT "one"
    CASE ELSE
        PRINT "other"
END SELECT
ON a GOSUB s
FOR i = 1 TO 0: PRINT i: NEXT i
DO: a = a + 1: LOOP WHILE a < 3
WHILE a < 5: a = a + 1: WEND
SYSTEM
s:
RETURN
