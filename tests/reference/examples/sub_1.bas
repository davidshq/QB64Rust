$CONSOLE:ONLY
n& = 1
Bump n&
PRINT "After Bump n&, n& ="; n&
Bump (n&)
PRINT "After Bump (n&), n& ="; n&
Greet "Ney", 2
END

SUB Bump (x AS LONG)
    x = x + 1
END SUB

SUB Greet (who AS STRING, times AS INTEGER)
    FOR i = 1 TO times
        PRINT "Hello, "; who
    NEXT i
END SUB
