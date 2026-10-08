' TEST: typed
$CONSOLE:ONLY
' SELECT CASE in the typed tree (m2-core-builtins task 6.1, measured verification\v20_h_select): a plain variable is
' read at each test, anything else copied once (LONG for an INTEGER expression, _INTEGER64, the believed float
' type, a string); items converted to the selector's type (a float item for an integer selector rounded to LONG or
' _INTEGER64), both sides in C's common type; IS with each operator, TO, lists, CASE ELSE, EVERYCASE
DIM i AS INTEGER, q AS _INTEGER64, f AS SINGLE, s AS STRING, a(3) AS INTEGER
SELECT CASE i
    CASE 1, 2.4: PRINT "a"
    CASE 3 TO 5.5: PRINT "b"
    CASE IS > q: PRINT "c"
    CASE ELSE: PRINT "d"
END SELECT
SELECT CASE a(1)
    CASE 1: PRINT "e"
END SELECT
SELECT CASE i + 1
    CASE 2.5: PRINT "f"
END SELECT
SELECT EVERYCASE f
    CASE 1, 2#: PRINT "g"
    CASE IS <> 0.1#: PRINT "h"
    CASE ELSE: PRINT "i"
END SELECT
SELECT CASE s + "x"
    CASE "a" TO "m", IS >= s: PRINT "j"
END SELECT
SELECT CASE 2.5
    CASE 2: PRINT "k"
END SELECT
SELECT CASE LEN(s)
END SELECT
