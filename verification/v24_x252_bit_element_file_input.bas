$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.2): INPUT # into elements of a _BIT array: values inside the
' element's range, then one outside it (9 into a _BIT * 4).
ON ERROR GOTO h
DIM b(3) AS _BIT * 4, u(3) AS _UNSIGNED _BIT * 3
OPEN "v24_x252.tmp" FOR OUTPUT AS #1
PRINT #1, "5, -8, 7, 9"
CLOSE #1
OPEN "v24_x252.tmp" FOR INPUT AS #1
INPUT #1, b(1), b(2), u(1)
PRINT b(0); b(1); b(2); b(3); u(0); u(1); u(2)
PRINT "9 into a _BIT * 4:";
INPUT #1, b(3)
PRINT b(3)
CLOSE #1
KILL "v24_x252.tmp"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
