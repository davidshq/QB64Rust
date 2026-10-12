' TEST: ir
$CONSOLE:ONLY
' SELECT CASE lowered (m2-core-builtins task 6.2, design D7): a copied selector is a global in the main module and
' a STATIC variable in a procedure (DIVERGENCES-QB45.md Q-002), a plain variable is read at each test; each CASE a
' Branch to the next one (Skip on a pending error, as IF), items joined by _ORELSE, a range by _ANDALSO; a Jump to
' the end after each body; EVERYCASE with its flag (a temporary) guarding CASE ELSE
x = 3
SELECT CASE x
    CASE 1, 2: PRINT "a"
    CASE 3 TO 4: PRINT "b"
    CASE ELSE: PRINT "c"
END SELECT
SELECT EVERYCASE x * 2
    CASE IS > 1: PRINT "d"
    CASE ELSE: PRINT "e"
END SELECT
PRINT f(1)
FUNCTION f (n)
    SELECT CASE n * 10
        CASE 10: f = 1
        CASE ELSE: f = 2
    END SELECT
END FUNCTION
