$CONSOLE:ONLY
DIM SHARED total AS LONG
total = 7
CALL show(5)
PRINT total
SYSTEM

SUB show (n AS LONG)
    total = total + n
END SUB
