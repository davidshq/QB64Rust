$CONSOLE:ONLY
' Verification (QB64pe issue 95): REDIM without AS of an array of a TYPE. Whether the TYPE array keeps its
' values and bounds, and what the SINGLE array a!() of the same name holds afterwards.
TYPE f
    foo AS INTEGER
END TYPE
ON ERROR GOTO h
REDIM a(2) AS f
a(1).foo = 7
REDIM a(5)
PRINT "a(1).foo"; a(1).foo
PRINT "lbound"; LBOUND(a); "ubound"; UBOUND(a)
PRINT "ubound a!"; UBOUND(a!)
a!(4) = 1.5
PRINT "a!(4)"; a!(4)
SYSTEM
h:
PRINT "error"; ERR; "at line"; _ERRORLINE
RESUME NEXT
