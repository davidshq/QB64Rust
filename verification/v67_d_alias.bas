$CONSOLE:ONLY
' Verification (_BIT parameters, QB64pe after fae85f8e1): a _BIT variable of the parameter's type is the parameter itself (a SHARED one seen both ways), from a SUB and from a FUNCTION, also when DECLAREd.
DECLARE SUB s (x AS _BIT * 5)
DIM SHARED g AS _BIT * 5
g = 3
s g
PRINT "after"; g
DIM b AS _BIT * 5
b = 3
s (b)
PRINT b
f = h(b)
PRINT b
DIM one AS _BIT
one = -1
z one
PRINT one
SYSTEM
SUB s (x AS _BIT * 5)
x = 5
PRINT "inside g="; g
g = 7
PRINT "inside x="; x
END SUB
FUNCTION h (x AS _BIT * 5)
x = 10
END FUNCTION
SUB z (x AS _BIT)
x = 0
END SUB
