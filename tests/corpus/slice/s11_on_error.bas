$CONSOLE:ONLY
' Error handlers (spec language/error-handling): RESUME NEXT, RESUME label, retry reaching a second handler,
' ERR before, inside and after a handler, ERROR with unusual values, errors raised inside a SUB and a FUNCTION,
' a handler set inside a SUB, ON ERROR GOTO 0, and a critical error that is not trapped (last).
ON ERROR GOTO handler
PRINT "ERR before:"; ERR
ERROR 5
PRINT "after RESUME NEXT, ERR:"; ERR
ERROR 0
ERROR -1
ERROR 2.5
ERROR 3.5
ERROR 70000
raises
PRINT "f:"; fraises&
ON ERROR GOTO tolabel
ERROR 6
PRINT "skipped by RESUME back"
back: PRINT "at back"
ON ERROR GOTO first
ERROR 7
PRINT "after retry"
sethandler
ERROR 9
ON ERROR GOTO 0
PRINT "handler removed"
ON ERROR GOTO handler
ERROR 11
PRINT "not reached: error 11 is critical"
SYSTEM

handler:
PRINT "handler: ERR"; ERR; "ERL"; ERL
RESUME NEXT

tolabel:
PRINT "tolabel: ERR"; ERR
RESUME back

first:
PRINT "first: ERR"; ERR
ON ERROR GOTO second
RESUME

second:
PRINT "second: ERR"; ERR
RESUME NEXT

subh:
PRINT "subh: ERR"; ERR
RESUME NEXT

SUB raises
    PRINT "in raises"
    ERROR 8
    PRINT "raises continues"
END SUB

FUNCTION fraises&
    fraises& = 1
    ERROR 7
    fraises& = 2
END FUNCTION

SUB sethandler
    ON ERROR GOTO subh
END SUB
