$CONSOLE:ONLY
' Verification (_BIT parameters, QB64pe after fae85f8e1): an element of a _BIT array of the parameter's width in parentheses is a copy.
DIM a(3) AS _BIT * 5
a(1) = 3
s (a(1))
PRINT a(1)
SYSTEM
SUB s (x AS _BIT * 5)
PRINT x
x = 9
END SUB
