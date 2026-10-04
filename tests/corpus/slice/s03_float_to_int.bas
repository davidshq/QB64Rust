$CONSOLE:ONLY
' Storing into an integer variable (numeric-semantics spec): half to even, SINGLE narrowing for INTEGER targets,
' truncation to the target width.
x% = 2.5: PRINT x%
x% = 3.5: PRINT x%
x% = -2.5: PRINT x%
x% = 0.5: PRINT x%
x% = 1.5: PRINT x%
l& = 2.5: PRINT l&
l& = 3.5: PRINT l&
q&& = 2.5#: PRINT q&&
q&& = -3.5: PRINT q&&
d# = 2.5000001: x% = d#: l& = d#
PRINT x%; l&
d# = 16777217.5#: l& = d#: PRINT l&
s! = 2.5: x% = s!: PRINT x%
x% = 32767.6: PRINT x%
x% = 98765.4: PRINT x%
l& = 3000000000#: PRINT l&
s! = 0.1#: PRINT s!
s! = 1.23456789#: PRINT s!
d# = s!: PRINT d#
s! = 7: PRINT s!
d# = 9223372036854775807: PRINT d#
END
