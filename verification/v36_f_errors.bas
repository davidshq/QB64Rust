$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): the run-time errors of a() = b(): bounds that
' differ (lower, upper, the same count moved, a second dimension), a target or source that is not allocated
' ($DYNAMIC never REDIMmed, after ERASE), a static array after ERASE; the target is unchanged after the error.
ON ERROR GOTO h
DIM i AS LONG
DIM a(0 TO 5) AS LONG, b(1 TO 6) AS LONG, c(0 TO 4) AS LONG, d(0 TO 6) AS LONG
DIM m(2, 3) AS LONG, n(2, 4) AS LONG
FOR i = 0 TO 5: a(i) = i + 1: NEXT
b(1) = 99: c(0) = 98: d(0) = 97
PRINT "1. same count, moved:";: b() = a(): PRINT b(1)
PRINT "2. fewer:";: c() = a(): PRINT c(0)
PRINT "3. more:";: d() = a(): PRINT d(0)
PRINT "4. second dimension:";: n() = m(): PRINT " done"
REDIM r(0 TO 5) AS LONG
ERASE r
PRINT "5. target erased:";: r() = a(): PRINT " done"
PRINT "6. source erased:";: a() = r(): PRINT a(0)
'$DYNAMIC
DIM nv(0 TO 5) AS LONG
'$STATIC
ERASE nv
PRINT "7. target never allocated again:";: nv() = a(): PRINT " done"
DIM st(0 TO 5) AS LONG
ERASE st
PRINT "8. static after ERASE:";: st() = a(): PRINT st(5)
DIM s1(2) AS STRING, s2(3) AS STRING
s2(0) = "keep"
PRINT "9. strings:";: s2() = s1(): PRINT " "; s2(0)
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
