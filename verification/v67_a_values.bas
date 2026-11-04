$CONSOLE:ONLY
' Verification (_BIT parameters, QB64pe after fae85f8e1): a value that is not a _BIT variable of the parameter's width is a copy, not cut to the width until stored.
s1 1
s1 -1
s1 2
s5 100
s5 -100
DIM l AS LONG
l = 100
s5 l
PRINT "l="; l
s5 (l)
SYSTEM
SUB s1 (x AS _BIT)
PRINT x;
x = x
PRINT x
END SUB
SUB s5 (x AS _BIT * 5)
PRINT x;
x = x
PRINT x
END SUB
