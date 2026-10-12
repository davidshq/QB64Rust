$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a label inside a FOR inside a SUB, reached by GOTO from outside the FOR (loop capped).
t
SYSTEM
SUB t
    GOTO inl
    FOR i = 1 TO 2
        PRINT "top"; i
        inl:
        n = n + 1: PRINT "in"; i
        IF n > 6 THEN PRINT "capped": EXIT FOR
    NEXT
    PRINT "after"; i
END SUB
