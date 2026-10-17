$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): member arrays of a plain TYPE (no $UNSTABLE) as source and
' destination: of a variable, of an element of an array, of a nested member; STRING * n members; the index of the
' element evaluated where.
TYPE t
    k AS INTEGER
    b(1 TO 6) AS LONG
    s(0 TO 2) AS STRING * 3
END TYPE
TYPE o
    tag AS LONG
    inner AS t
END TYPE
DIM i AS LONG
DIM v AS t, tt(1 TO 2) AS t, oo AS o, oa(0 TO 1) AS o
DIM src(1 TO 4) AS LONG, dst(0 TO 7) AS LONG
FOR i = 1 TO 4: src(i) = i * 11: NEXT

_ARRAYCOPY src() TO v.b(2)
PRINT "1. array to a member of a variable:";: FOR i = 1 TO 6: PRINT v.b(i);: NEXT: PRINT
_ARRAYCOPY src(2 TO 3) TO tt(2).b(5)
PRINT "2. range to a member of an element:";: FOR i = 1 TO 6: PRINT tt(2).b(i);: NEXT: PRINT
PRINT "   the other element:";: FOR i = 1 TO 6: PRINT tt(1).b(i);: NEXT: PRINT
_ARRAYCOPY v.b() TO dst()
PRINT "3. member whole to whole:";: FOR i = 0 TO 7: PRINT dst(i);: NEXT: PRINT
_ARRAYCOPY v.b(2 TO 4) TO oo.inner.b()
PRINT "4. member to a nested member:";: FOR i = 1 TO 6: PRINT oo.inner.b(i);: NEXT: PRINT
_ARRAYCOPY oo.inner.b() TO oa(1).inner.b(1)
PRINT "5. nested to nested of an element:";: FOR i = 1 TO 6: PRINT oa(1).inner.b(i);: NEXT: PRINT
PRINT "   neighbours:"; oa(1).tag; oa(1).inner.k; oa(0).inner.b(1)
v.s(0) = "abc": v.s(1) = "def": v.s(2) = "ghi"
_ARRAYCOPY v.s(1 TO 2) TO tt(1).s(0)
PRINT "6. STRING * 3 members:"; tt(1).s(0); tt(1).s(1); "["; tt(1).s(2); "]"
DIM fs(0 TO 2) AS STRING * 3
_ARRAYCOPY v.s() TO fs()
PRINT "7. member to STRING * 3 array:"; fs(0); fs(1); fs(2)
FOR i = 1 TO 6: v.b(i) = i: NEXT
_ARRAYCOPY v.b(1 TO 4) TO v.b(3)
PRINT "8. member overlap:";: FOR i = 1 TO 6: PRINT v.b(i);: NEXT: PRINT
_ARRAYCOPY src() TO tt(ix(2)).b(ix(1))
PRINT "9. indexes by functions:";: FOR i = 1 TO 6: PRINT tt(2).b(i);: NEXT: PRINT
_ARRAYCOPY tt(ix(1)).b(ix(2) TO ix(3)) TO dst(ix(0))
PRINT "10. source member by functions:";: FOR i = 0 TO 7: PRINT dst(i);: NEXT: PRINT
SYSTEM

FUNCTION ix& (n AS LONG)
    PRINT "(ix"; n; ")";
    ix& = n
END FUNCTION
