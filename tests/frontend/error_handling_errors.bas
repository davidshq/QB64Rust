' TEST: check-fail
$CONSOLE:ONLY
' Label and error-handling errors (m2-procedures-and-errors D8; old compiler's messages in verification\v14_err_*)
a:
a:
ON ERROR GOTO nowhere
RESUME nowhere
ON ERROR GOTO 10
RESUME 10
ON ERROR GOTO a$
ERROR "x"
PRINT CHR$("a")
PRINT CHR$(1, 2)
cls: PRINT 1
s: PRINT 2
PRINT ERR(1)
SUB s (k AS LONG)
    ON ERROR GOTO a
    RESUME a
    RESUME NEXT
    inner:
END SUB
