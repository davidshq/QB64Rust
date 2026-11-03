$CONSOLE:ONLY
' Verification (_BIT parameters, QB64pe after fae85f8e1): a parameter of more than 32 bits, by reference and as a copy, and its store cut to the width.
DIM w AS _BIT * 33
w = 123456789
s w
PRINT w
DIM u AS _UNSIGNED _BIT * 40
u = 1099511627775
t u
PRINT u
s 5000000000
SYSTEM
SUB s (x AS _BIT * 33)
PRINT x;
x = x + 1
PRINT x
END SUB
SUB t (x AS _UNSIGNED _BIT * 40)
x = x + 1
END SUB
