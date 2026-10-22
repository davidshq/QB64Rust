$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): _ARCSEC, _ARCCSC, _ARCCOT, _SECH and _CSCH at run time: values
' of each type of argument, the edges of each domain, and the arguments outside it under a handler that prints
' ERR and the value returned.
ON ERROR GOTO h
x# = 1.2
PRINT _ARCSEC(x#); _ARCCSC(x#); _ARCCOT(x#); _SECH(x#); _CSCH(x#)
y! = -2.5
PRINT _ARCSEC(y!); _ARCCSC(y!); _ARCCOT(y!); _SECH(y!); _CSCH(y!)
i% = 3
PRINT _ARCSEC(i%); _ARCCSC(i%); _ARCCOT(i%); _SECH(i%); _CSCH(i%)
f## = 1.5
PRINT _ARCSEC(f##); _ARCCSC(f##); _ARCCOT(f##); _SECH(f##); _CSCH(f##)
PRINT "edges"
PRINT _ARCSEC(1); _ARCSEC(-1); _ARCCSC(1); _ARCCSC(-1)
PRINT _ARCCOT(0); _ARCCOT(1E+300); _ARCCOT(-1E+300)
PRINT _SECH(0); _SECH(88); _SECH(-88); _SECH(-1000)
PRINT _CSCH(88); _CSCH(-88); _CSCH(-1000)
PRINT "outside"
r# = 7: r# = _ARCSEC(0.5): PRINT "arcsec(0.5)"; r#
r# = 7: r# = _ARCSEC(0): PRINT "arcsec(0)"; r#
r# = 7: r# = _ARCCSC(-0.5): PRINT "arccsc(-0.5)"; r#
r# = 7: r# = _SECH(88.1): PRINT "sech(88.1)"; r#
r# = 7: r# = _SECH(1000): PRINT "sech(1000)"; r#
r# = 7: r# = _CSCH(0): PRINT "csch(0)"; r#
r# = 7: r# = _CSCH(88.1): PRINT "csch(88.1)"; r#
PRINT "types"
a! = _ARCCOT(1): b# = _ARCCOT(1): c## = _ARCCOT(1)
PRINT a!; b#; c##
PRINT LEN(STR$(_SECH(1))); LEN(STR$(_ARCCOT(2)))
SYSTEM
h:
PRINT "ERR"; ERR
RESUME NEXT
