$CONSOLE:ONLY
' Integer arithmetic width and wrap (numeric-semantics spec; DIVERGENCES.md D-001, D-002).
i% = 32767
PRINT i% + 1
i% = i% + 1
PRINT i%
x% = 70000
PRINT x%
c% = 200: d% = 200
PRINT c% * d%
e% = c% * d%
PRINT e%
PRINT 200 * 200
l& = 2147483647
PRINT l& + 1
l& = l& + 1
PRINT l&
PRINT 2147483647 * 2
PRINT 2147483647 + 1
PRINT 50000 * 50000
PRINT -2147483647 - 2
q&& = 9223372036854775807
PRINT q&&
PRINT q&& + 1
q&& = q&& + 1
PRINT q&&
PRINT 2147483648 * 2
PRINT 3037000500 * 3037000500
l& = 50000: PRINT l& * l&; l& * 50000&&
x% = -32768: PRINT -x%; -(-32768)
l& = -2147483647 - 1: PRINT -l&
END
