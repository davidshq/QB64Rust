$CONSOLE:ONLY
PRINT CINT(3.7); CINT(-3.7); CINT(2.5); CINT(3.5)
ON ERROR GOTO toobig
x% = 7
x% = CINT(40000)
PRINT "x% is"; x%
END

toobig:
PRINT "Error"; ERR; "(Overflow)"
RESUME NEXT
