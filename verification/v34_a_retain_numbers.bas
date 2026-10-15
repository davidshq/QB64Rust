$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): REDIM _RETAIN of numbers: one dimension moved, grown and
' shrunk, two and three dimensions, ranges that do not meet, the first REDIM _RETAIN of an array, after ERASE,
' the type kept without AS, a suffix, INTEGER, DOUBLE, _INTEGER64, _UNSIGNED _BYTE.
ON ERROR GOTO h
DIM i AS LONG, j AS LONG, k AS LONG
PRINT "1. a(1 TO 5) = 10..50, then a(3 TO 7):";
REDIM a(1 TO 5) AS LONG
FOR i = 1 TO 5: a(i) = i * 10: NEXT
REDIM _RETAIN a(3 TO 7) AS LONG
FOR i = 3 TO 7: PRINT a(i);: NEXT: PRINT
PRINT "2. grown to a(0 TO 9):";
REDIM _RETAIN a(0 TO 9) AS LONG
FOR i = 0 TO 9: PRINT a(i);: NEXT: PRINT
PRINT "3. shrunk to a(4 TO 5):";
REDIM _RETAIN a(4 TO 5) AS LONG
FOR i = 4 TO 5: PRINT a(i);: NEXT: PRINT
PRINT "4. moved away, a(100 TO 102):";
REDIM _RETAIN a(100 TO 102) AS LONG
FOR i = 100 TO 102: PRINT a(i);: NEXT: PRINT
PRINT "5. two dimensions, m(1 TO 3, 1 TO 4) = 10*i+j, then m(2 TO 5, 0 TO 3):"
REDIM m(1 TO 3, 1 TO 4) AS INTEGER
FOR i = 1 TO 3: FOR j = 1 TO 4: m(i, j) = 10 * i + j: NEXT: NEXT
REDIM _RETAIN m(2 TO 5, 0 TO 3) AS INTEGER
FOR i = 2 TO 5: FOR j = 0 TO 3: PRINT m(i, j);: NEXT: PRINT: NEXT
PRINT "6. three dimensions, c(2, 2, 2) = 100*i+10*j+k, then c(1 TO 3, 0 TO 1, 1 TO 2):"
REDIM c(2, 2, 2) AS DOUBLE
FOR i = 0 TO 2: FOR j = 0 TO 2: FOR k = 0 TO 2: c(i, j, k) = 100 * i + 10 * j + k + 0.5: NEXT: NEXT: NEXT
REDIM _RETAIN c(1 TO 3, 0 TO 1, 1 TO 2) AS DOUBLE
FOR i = 1 TO 3: FOR j = 0 TO 1: FOR k = 1 TO 2: PRINT c(i, j, k);: NEXT: NEXT: PRINT: NEXT
PRINT "7. the first REDIM _RETAIN of an array, f(2 TO 4):";
REDIM _RETAIN f(2 TO 4) AS LONG
PRINT LBOUND(f); UBOUND(f); f(2); f(4)
PRINT "8. after ERASE:";
f(3) = 9
ERASE f
REDIM _RETAIN f(1 TO 5) AS LONG
PRINT LBOUND(f); UBOUND(f); f(3)
PRINT "9. the type kept without AS, a suffix, _INTEGER64:";
REDIM q(5) AS _INTEGER64
q(5) = 123456789012345
REDIM _RETAIN q(3 TO 8)
PRINT q(5); LEN(q(5))
REDIM s&(3)
s&(2) = 77
REDIM _RETAIN s&(2 TO 6)
PRINT s&(2)
PRINT "10. _UNSIGNED _BYTE, b(0 TO 7) = 0..7, then b(-2 TO 3):";
REDIM b(0 TO 7) AS _UNSIGNED _BYTE
FOR i = 0 TO 7: b(i) = i: NEXT
REDIM _RETAIN b(-2 TO 3) AS _UNSIGNED _BYTE
FOR i = -2 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "11. bounds that give no element, a(5 TO j) with j = 4:";
REDIM a(1 TO 3) AS LONG: a(2) = 4
j = 4
REDIM _RETAIN a(5 TO j) AS LONG
PRINT " then:"; LBOUND(a); UBOUND(a)
REDIM _RETAIN a(1 TO 3) AS LONG
PRINT " a(1 TO 3) again:"; a(1); a(2); a(3)
PRINT "12. same bounds:";
a(2) = 6
REDIM _RETAIN a(1 TO 3) AS LONG
PRINT a(2)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
