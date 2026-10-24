$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and the critical error in a SUB of the main file, and the
' line of a statement in a SUB after a FUNCTION (the lines are the file's).
$ERRORLOCATION:ON
PRINT "start"
divide 7
PRINT "not reached"
SYSTEM

FUNCTION f (n)
    f = n
END FUNCTION

SUB divide (n AS INTEGER)
    z% = 0
    PRINT "in sub"
    q% = n \ z%
END SUB
