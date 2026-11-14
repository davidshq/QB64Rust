$CONSOLE:ONLY
t# = TIMER(.001)
FOR i = 1 TO 10
    _LIMIT 20
NEXT
PRINT "Ten rounds at 20 a second took over 0.3 seconds:"; Taken#(t#) > .3
END

' The seconds since t#, also across midnight.
FUNCTION Taken# (t#)
    e# = TIMER(.001) - t#
    IF e# < 0 THEN e# = e# + 86400
    Taken# = e#
END FUNCTION
