$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): arrays of _BIT, _UNSIGNED _BIT and _BIT * n: static and dynamic,
' values at their limits, elements that share a byte, two dimensions, an index outside, ERASE, REDIM and REDIM
' _PRESERVE.
ON ERROR GOTO h
DIM i AS LONG, n AS LONG
DIM b(7) AS _BIT
b(1) = 1: b(3) = -1: b(4) = 2
PRINT "1. _BIT, b(1) = 1, b(3) = -1, b(4) = 2: b(0) to b(4)"; b(0); b(1); b(2); b(3); b(4)
DIM u(7) AS _UNSIGNED _BIT
u(1) = 1: u(3) = -1: u(4) = 2
PRINT "2. _UNSIGNED _BIT, the same stores:"; u(0); u(1); u(2); u(3); u(4)
DIM b4(3) AS _BIT * 4
b4(0) = 7: b4(1) = 8: b4(2) = -8: b4(3) = 15
PRINT "3. _BIT * 4, stores 7, 8, -8, 15:"; b4(0); b4(1); b4(2); b4(3)
DIM u3(9) AS _UNSIGNED _BIT * 3
FOR i = 0 TO 9: u3(i) = i: NEXT
PRINT "4. _UNSIGNED _BIT * 3, u3(i) = i for 0 to 9:";
FOR i = 0 TO 9: PRINT u3(i);: NEXT
PRINT
u3(4) = 0
PRINT "   after u3(4) = 0, u3(3) to u3(5):"; u3(3); u3(4); u3(5)
n = 9
DIM dy(n) AS _BIT * 5
FOR i = 0 TO 9: dy(i) = i - 5: NEXT
PRINT "5. dynamic _BIT * 5, dy(i) = i - 5:";
FOR i = 0 TO 9: PRINT dy(i);: NEXT
PRINT
DIM m(1, 2) AS _BIT * 3
m(1, 2) = 3: m(0, 1) = -4
PRINT "6. two dimensions, _BIT * 3:"; m(1, 2); m(0, 1); m(1, 1)
PRINT "7. an index outside, b(8) = 1 and PRINT b4(4):";
b(8) = 1
PRINT b4(4)
ERASE b4
PRINT "8. after ERASE b4:"; b4(0); b4(1); b4(2); b4(3)
REDIM r(4) AS _BIT * 5
FOR i = 0 TO 4: r(i) = i + 1: NEXT
REDIM _PRESERVE r(9) AS _BIT * 5
PRINT "9. REDIM _PRESERVE from 4 to 9, _BIT * 5:";
FOR i = 0 TO 9: PRINT r(i);: NEXT
PRINT
REDIM r(2) AS _BIT * 5
PRINT "10. REDIM r(2):"; r(0); r(1); r(2)
PRINT "11. b4(1) + 1 after b4(1) = 7, and 2.5 and 3.5 stored:";
b4(1) = 7: b4(2) = 2.5: b4(3) = 3.5
PRINT b4(1) + 1; b4(2); b4(3)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
