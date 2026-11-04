$CONSOLE:ONLY
' Verification (_BIT parameters, QB64pe after fae85f8e1): by reference across _UNSIGNED of one width; a copy for another width, an element of another width, a LONG, and in parentheses.
DIM b AS _BIT * 3
b = 1
t b
PRINT b
DIM a(3) AS _BIT * 5
a(1) = 3
w a(1)
PRINT a(1)
l a(1)
PRINT a(1)
DIM n AS LONG
n = 2
w n
PRINT n
w (b)
PRINT b
DIM u AS _UNSIGNED _BIT * 3
u = 7
v u
PRINT u
SYSTEM
SUB t (x AS _UNSIGNED _BIT * 3)
x = 6
END SUB
SUB w (x AS _BIT * 3)
PRINT "w"; x
x = 1
END SUB
SUB l (x AS LONG)
x = 9
END SUB
SUB v (x AS _UNSIGNED _BIT * 3)
x = x + 1
END SUB
