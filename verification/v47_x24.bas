$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): _CONTINUE in a loop inside a GOSUB subroutine of the main module.
GOSUB r
PRINT "end"
SYSTEM
r:
FOR i = 1 TO 3
    IF i = 2 THEN _CONTINUE
    PRINT i;
NEXT
PRINT
RETURN
