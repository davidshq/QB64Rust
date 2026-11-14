$CONSOLE:ONLY
DECLARE SUB Report (title AS STRING, n AS INTEGER)
DECLARE FUNCTION Square& (x AS LONG)
Report "Squares", 3
END

SUB Report (title AS STRING, n AS INTEGER)
    PRINT title
    FOR i& = 1 TO n
        PRINT i&; Square&(i&)
    NEXT i&
END SUB

FUNCTION Square& (x AS LONG)
    Square& = x * x
END FUNCTION
