$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a scalar assigned a whole TYPE value.
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
y& = p
PRINT y&
SYSTEM
