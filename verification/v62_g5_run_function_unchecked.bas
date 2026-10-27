$CONSOLE:ONLY
' Verification (fixes-switch, review): study 39 row 14, RUN in a FUNCTION called from a loop below $CHECKING:OFF
' whose statements call no SUB. The count is kept in the process's environment, which RUN does not clear.
c = VAL(ENVIRON$("V62_G5_COUNT")) + 1
ENVIRON "V62_G5_COUNT=" + LTRIM$(STR$(c))
PRINT "run"; c
IF c > 1 THEN PRINT "restarted": SYSTEM
$CHECKING:OFF
DO
    x& = f&(1)
LOOP
$CHECKING:ON
SYSTEM

FUNCTION f& (a AS LONG)
    RUN
END FUNCTION
