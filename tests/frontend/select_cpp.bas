' TEST: cpp
$CONSOLE:ONLY
' SELECT CASE as emitted (m2-core-builtins task 6.2): the copy's declaration (global in the main module, a STATIC
' pointer of the procedure), the tests with C's || and && through -(a||b) and -(a&&b), string comparisons through
' the runtime with the statement's string cleanup, EVERYCASE's flag
DIM s AS STRING
x = 3
SELECT CASE x
    CASE 1, 2: PRINT "a"
    CASE 3 TO 4: PRINT "b"
    CASE ELSE: PRINT "c"
END SELECT
SELECT EVERYCASE s + "x"
    CASE "ax": PRINT "d"
    CASE ELSE: PRINT "e"
END SELECT
sp 1
SUB sp (n)
    SELECT CASE n * 10
        CASE 10: PRINT "f"
    END SELECT
END SUB
