$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): an omitted argument: f(, 2, 3) of a program FUNCTION.
u = f(, 2, 3)
PRINT "ok"
SYSTEM
FUNCTION f (a, b, c)
    f = a + b + c
END FUNCTION
