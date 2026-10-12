$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): more spellings of a constant bound, as v24_b_constant_bounds: a
' minus before a CONST, before parentheses and inside them, nested parentheses, a CONST with a suffix, a CONST
' defined by an expression, and what a DIM with such a bound does when GOTO skips it.
ON ERROR GOTO h
CONST k = 5
CONST e = 2 + 3
DIM b1(-k TO 5) AS LONG
PRINT "1. -k TO 5:";
REDIM b1(20) AS LONG
PRINT UBOUND(b1)
DIM b2((-1) TO 5) AS LONG
PRINT "2. (-1) TO 5:";
REDIM b2(20) AS LONG
PRINT UBOUND(b2)
DIM b3(-(1) TO 5) AS LONG
PRINT "3. -(1) TO 5:";
REDIM b3(20) AS LONG
PRINT UBOUND(b3)
DIM b4(((5))) AS LONG
PRINT "4. ((5)):";
REDIM b4(20) AS LONG
PRINT UBOUND(b4)
DIM b5(k&) AS LONG
PRINT "5. k&:";
REDIM b5(20) AS LONG
PRINT UBOUND(b5)
DIM b6(e) AS LONG
PRINT "6. a CONST defined as 2 + 3:";
REDIM b6(20) AS LONG
PRINT UBOUND(b6)
DIM b7(- 1 TO 5) AS LONG
PRINT "7. - 1 TO 5 (a blank after the minus):";
REDIM b7(20) AS LONG
PRINT UBOUND(b7)
DIM b8(--1 TO 5) AS LONG
PRINT "8. --1 TO 5:";
REDIM b8(20) AS LONG
PRINT UBOUND(b8)
GOTO skip
DIM c1(2 + 3) AS LONG
DIM c2(5) AS LONG
skip:
PRINT "9. DIM c1(2 + 3) skipped by GOTO, c1(1) = 1:";
c1(1) = 1
PRINT
PRINT "10. DIM c2(5) skipped by GOTO, c2(1) = 1:";
c2(1) = 1
PRINT c2(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
