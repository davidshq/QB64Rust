$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED a, without parentheses, for a main array: the element
' then named in the SUB.
ON ERROR GOTO h
DIM a(3) AS LONG
a(2) = 8
s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    SHARED a AS LONG
    PRINT a; a(2)
END SUB
