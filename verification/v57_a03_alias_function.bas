$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DECLARE FUNCTION f ALIAS t (x) outside a library.
DECLARE FUNCTION f ALIAS t (x)
PRINT f(2)
SYSTEM
FUNCTION f (x)
    f = x + 1
END FUNCTION
