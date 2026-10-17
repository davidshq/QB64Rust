$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (50_in_function)
x = f
PRINT "ok"
SYSTEM
h:
RETURN
FUNCTION f
    ON TIMER(1) GOSUB h
END FUNCTION
