$CONSOLE:ONLY
' Division (numeric-semantics spec): integer / integer in _FLOAT printed with 16 digits; otherwise the wider float.
PRINT 1 / 3
PRINT 2 / 3
PRINT 7 / 2
PRINT 10 / 4
d# = 2.5: PRINT d# / 3
s! = 2.5: PRINT s! / 3
PRINT 1! / 3
PRINT 2 / 3!
PRINT 2 / 3#
a% = 1: b% = 3
PRINT a% / b%
s! = a% / b%: PRINT s!
d# = a% / b%: PRINT d#
x% = 7 / 2: PRINT x%
l& = 2147483647: PRINT l& / 2
q&& = 9223372036854775807: PRINT q&& / 2
PRINT 1 / 3 * 3
PRINT (1 + 2) / (4 - 1)
END
