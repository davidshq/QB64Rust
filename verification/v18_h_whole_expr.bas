$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a whole TYPE value in an expression.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
y = p + 1
PRINT y
SYSTEM
