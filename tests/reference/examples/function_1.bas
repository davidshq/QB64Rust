$CONSOLE:ONLY
PRINT "Twice 21 is"; Twice&(21)
PRINT "The larger of 3 and 8 is"; Larger(3, 8)
PRINT "["; Pad$("ab", 5); "]"
END

FUNCTION Twice& (a AS LONG)
    Twice& = a * 2
END FUNCTION

FUNCTION Larger (a, b)
    IF a > b THEN Larger = a ELSE Larger = b
END FUNCTION

FUNCTION Pad$ (s AS STRING, w AS INTEGER)
    Pad$ = s + SPACE$(w - LEN(s))
END FUNCTION
