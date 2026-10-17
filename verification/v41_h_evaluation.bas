$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): the order in which the bounds, the start and the indexes on
' the way to member arrays are evaluated, each once; bounds that are not whole numbers (rounded how), of other
' types, in parentheses; an error raised in a parent index; the statement after a raised error.
TYPE t
    b(1 TO 4) AS LONG
END TYPE
DIM i AS LONG
DIM a(0 TO 9) AS LONG, d(0 TO 9) AS LONG, tt(1 TO 2) AS t
FOR i = 0 TO 9: a(i) = i: NEXT
PRINT "1. order:";: _ARRAYCOPY a(f(2) TO f(4)) TO d(f(6)): PRINT: GOSUB show
PRINT "2. order with members:";: _ARRAYCOPY tt(f(1)).b(f(2) TO f(3)) TO tt(f(2)).b(f(1)): PRINT
PRINT "   "; tt(2).b(1); tt(2).b(2)
PRINT "3. 2D order:";
DIM m(0 TO 3, 0 TO 3) AS LONG, n(0 TO 3, 0 TO 3) AS LONG
_ARRAYCOPY m(f(1) TO f(2), f(3) TO f(3)) TO n(f(0), f(1)): PRINT
GOSUB clr: _ARRAYCOPY a(1.5 TO 3.5) TO d(0.5): PRINT "4. halves:": GOSUB show
GOSUB clr: _ARRAYCOPY a(2.4 TO 2.6) TO d(4.49): PRINT "5. near halves:": GOSUB show
GOSUB clr: _ARRAYCOPY a((1) TO (1 + 2)) TO d((5)): PRINT "6. parentheses:": GOSUB show
DIM b8 AS _BYTE, u64 AS _UNSIGNED _INTEGER64, sg AS SINGLE
b8 = 2: u64 = 3: sg = 4.5
GOSUB clr: _ARRAYCOPY a(b8 TO u64) TO d(sg): PRINT "7. other types:": GOSUB show
GOSUB clr: _ARRAYCOPY a(-1 + 2 TO 2 ^ 2) TO d(): PRINT "8. operators:": GOSUB show
ON ERROR GOTO h
PRINT "9. parent index outside:";: _ARRAYCOPY a(1 TO 2) TO tt(3).b(1): PRINT " done"
PRINT "10. source parent index outside:";: _ARRAYCOPY tt(0).b() TO d(): PRINT " done"
GOSUB clr: PRINT "11. error in the start, then the copy:";: _ARRAYCOPY a(1 TO 2) TO d(bad&): PRINT: GOSUB show
SYSTEM
clr:
FOR i = 0 TO 9: d(i) = -1: NEXT
RETURN
show:
FOR i = 0 TO 9: PRINT d(i);: NEXT
PRINT
RETURN
h:
PRINT " error"; ERR;
RESUME NEXT

FUNCTION f& (n AS LONG)
    PRINT " f"; LTRIM$(STR$(n));
    f& = n
END FUNCTION

FUNCTION bad&
    ERROR 5
    bad& = 0
END FUNCTION
