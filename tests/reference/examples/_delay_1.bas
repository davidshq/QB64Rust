$CONSOLE:ONLY
t# = TIMER(.001)
_DELAY 0.5
PRINT "_DELAY 0.5 waited over 0.4 seconds:"; Taken#(t#) > .4
END

' The seconds since t#, also across midnight.
FUNCTION Taken# (t#)
    e# = TIMER(.001) - t#
    IF e# < 0 THEN e# = e# + 86400
    Taken# = e#
END FUNCTION
