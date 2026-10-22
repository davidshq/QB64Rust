$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): which element a whole array a() stands for as a value:
' a lower bound other than 0, two dimensions, STRING, STRING * n; SWAP of two
' whole arrays of STRING; INPUT # and READ into one; PRINT USING and _IIF.
TYPE tm
    m(3) AS LONG
END TYPE
DIM a(5 TO 7) AS LONG
a(5) = 55: a(6) = 66: a(7) = 77
PRINT "1."; a()
DIM b(1 TO 2, 1 TO 3) AS INTEGER
b(1, 1) = 11: b(2, 1) = 21: b(1, 2) = 12
PRINT "2."; b()
DIM s(2) AS STRING
s(0) = "zero": s(1) = "one"
PRINT "3. ["; s(); "]"
DIM f(2) AS STRING * 4
f(0) = "abcd": f(1) = "efgh"
PRINT "4. ["; f(); "]"
DIM t AS tm
t.m(0) = 100: t.m(1) = 101
PRINT "5."; t.m(0)
DIM x(2) AS STRING, y(2) AS STRING
x(0) = "x0": x(1) = "x1": y(0) = "y0": y(1) = "y1"
SWAP x(), y()
PRINT "6. "; x(0); " "; x(1); " "; y(0); " "; y(1)
DIM r(1 TO 3) AS LONG
DATA 7,8
READ r()
PRINT "7."; r(1); r(2); r(3)
DIM ss(2) AS STRING
READ ss()
PRINT "8. ["; ss(0); "] ["; ss(1); "]"
DATA "str"
OPEN "v54_c_whole_values.tmp" FOR OUTPUT AS #1
PRINT #1, "41"
PRINT #1, "42"
CLOSE #1
DIM n(2 TO 4) AS LONG
OPEN "v54_c_whole_values.tmp" FOR INPUT AS #1
INPUT #1, n()
CLOSE #1
KILL "v54_c_whole_values.tmp"
PRINT "9."; n(2); n(3); n(4)
PRINT USING "10. ### ###"; a(); b()
PRINT "11."; _IIF(1, a(), a(6)); _IIF(0, a(6), a())
DIM g(2) AS DOUBLE
g(0) = 1.5
PRINT "12."; g()
PRINT "ok"
SYSTEM
