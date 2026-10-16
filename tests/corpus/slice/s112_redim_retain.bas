$CONSOLE:ONLY
' Slice (m3-redim-retain task 3.2): REDIM _RETAIN of numbers (verification\v34_a). The elements whose indexes lie
' in both the old and the new bounds keep their values at the same indexes: one dimension moved, grown, shrunk and
' moved away, two and three dimensions, the first REDIM _RETAIN of an array and one after ERASE, the type kept
' without AS and by a suffix, _UNSIGNED _BYTE, a dimension with no element (error 5), the same bounds.
ON ERROR GOTO h
DIM i AS LONG, j AS LONG, k AS LONG
PRINT "moved:";
REDIM a(1 TO 5) AS LONG
FOR i = 1 TO 5: a(i) = i * 10: NEXT
REDIM _RETAIN a(3 TO 7) AS LONG
FOR i = 3 TO 7: PRINT a(i);: NEXT: PRINT
PRINT "grown:";
REDIM _RETAIN a(0 TO 9) AS LONG
FOR i = 0 TO 9: PRINT a(i);: NEXT: PRINT
PRINT "shrunk:";
REDIM _RETAIN a(4 TO 5) AS LONG
FOR i = 4 TO 5: PRINT a(i);: NEXT: PRINT
PRINT "moved away:";
REDIM _RETAIN a(100 TO 102) AS LONG
FOR i = 100 TO 102: PRINT a(i);: NEXT: PRINT
PRINT "two dimensions:"
REDIM m(1 TO 3, 1 TO 4) AS INTEGER
FOR i = 1 TO 3: FOR j = 1 TO 4: m(i, j) = 10 * i + j: NEXT: NEXT
REDIM _RETAIN m(2 TO 5, 0 TO 3) AS INTEGER
FOR i = 2 TO 5: FOR j = 0 TO 3: PRINT m(i, j);: NEXT: PRINT: NEXT
PRINT "three dimensions:"
REDIM c(2, 2, 2) AS DOUBLE
FOR i = 0 TO 2: FOR j = 0 TO 2: FOR k = 0 TO 2: c(i, j, k) = 100 * i + 10 * j + k + 0.5: NEXT: NEXT: NEXT
REDIM _RETAIN c(1 TO 3, 0 TO 1, 1 TO 2) AS DOUBLE
FOR i = 1 TO 3: FOR j = 0 TO 1: FOR k = 1 TO 2: PRINT c(i, j, k);: NEXT: NEXT: PRINT: NEXT
PRINT "first:";
REDIM _RETAIN f(2 TO 4) AS LONG
PRINT LBOUND(f); UBOUND(f); f(2); f(4)
PRINT "after ERASE:";
f(3) = 9
ERASE f
REDIM _RETAIN f(1 TO 5) AS LONG
PRINT LBOUND(f); UBOUND(f); f(3)
PRINT "type kept:";
REDIM q(5) AS _INTEGER64
q(5) = 123456789012345
REDIM _RETAIN q(3 TO 8)
PRINT q(5); LEN(q(5));
REDIM s&(3)
s&(2) = 77
REDIM _RETAIN s&(2 TO 6)
PRINT s&(2)
PRINT "bytes:";
REDIM b(0 TO 7) AS _UNSIGNED _BYTE
FOR i = 0 TO 7: b(i) = i: NEXT
REDIM _RETAIN b(-2 TO 3) AS _UNSIGNED _BYTE
FOR i = -2 TO 3: PRINT b(i);: NEXT: PRINT
PRINT "no element:";
REDIM a(1 TO 3) AS LONG: a(2) = 4
j = 4
REDIM _RETAIN a(5 TO j) AS LONG
PRINT LBOUND(a); UBOUND(a);
REDIM _RETAIN a(1 TO 3) AS LONG
PRINT a(1); a(2); a(3)
PRINT "same bounds:";
a(2) = 6
REDIM _RETAIN a(1 TO 3) AS LONG
PRINT a(2)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
