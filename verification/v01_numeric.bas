$CONSOLE:ONLY
' Verification: division, overflow, literals, rounding, power, float comparison.
' Claims from study\02 sections 1.2-1.5 and section 7.

PRINT "== int/int division =="
PRINT 1 / 3
PRINT 2 / 3
PRINT 10 / 4
a% = 1: b% = 3
PRINT a% / b%
s! = a% / b%
PRINT s!
PRINT 1! / 3

PRINT "== integer overflow =="
a% = 32767: a% = a% + 1
PRINT a%
x% = 70000
PRINT x%
l& = 2147483647: l& = l& + 1
PRINT l&
c% = 200: d% = 200
PRINT c% * d%
e% = c% * d%
PRINT e%

PRINT "== single literal stored in double =="
d# = 0.1
PRINT d#
d# = 0.1!
PRINT d#
s! = 0.1: d# = s!
PRINT d#

PRINT "== float comparison narrowing =="
s! = 2.1
IF s! = 2.1 THEN PRINT "s! = 2.1 true" ELSE PRINT "s! = 2.1 false"
d# = 2.1
IF s! = d# THEN PRINT "s! = d# true" ELSE PRINT "s! = d# false"
IF d# = 2.1# THEN PRINT "d# = 2.1# true" ELSE PRINT "d# = 2.1# false"

PRINT "== rounding =="
PRINT CINT(0.5); CINT(1.5); CINT(2.5); CINT(3.5); CINT(-2.5)
x% = 2.5: PRINT x%
x% = 3.5: PRINT x%
l& = 2.5: PRINT l&
d# = 2.5: x% = d#: PRINT x%
PRINT CLNG(2.5); CLNG(3.5)
PRINT _ROUND(2.5); _ROUND(3.5)
' DOUBLE narrowed to SINGLE before rounding into INTEGER
d# = 16777217.5#: l& = d#: PRINT l&

PRINT "== hex and literal typing =="
PRINT &HFFFF; &H8000; &HFFFF&; &H8000&
PRINT &HFFFFFFFF; &H80000000
x% = &HFFFF: PRINT x%
PRINT -2147483648
' If -2147483648 is typed && the product is real 64-bit; if typed & it wraps in C int.
PRINT -2147483648 * 2
PRINT -2147483647 * 2
PRINT 2147483647 * 2
PRINT 2147483648 * 2

PRINT "== power =="
PRINT 2 ^ 3 ^ 2
PRINT -2 ^ 2
PRINT 2 ^ -1
CONST c1 = 2 ^ 3 ^ 2
PRINT c1
CONST c2 = -2 ^ 2
PRINT c2

PRINT "== DOUBLE narrowed to SINGLE before rounding into INTEGER =="
d# = 2.5000001#: x% = d#: PRINT x%
d# = 2.5000001#: l& = d#: PRINT l&
SYSTEM
