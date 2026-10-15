$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): SYSTEM 6 whose expression raises an error that is not trapped.
PRINT "x"
SYSTEM f
FUNCTION f
    ERROR 5
    f = 9
END FUNCTION
