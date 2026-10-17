$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): the run-time errors of every form: low above high, a range
' outside the source, a start outside the destination, a destination too small, an array not allocated or
' erased, a whole multidimensional copy with other bounds, a whole-to-start that does not fit, strings; the
' destination is unchanged after each error.
ON ERROR GOTO h
DIM i AS LONG
DIM a(1 TO 5) AS LONG, d(0 TO 5) AS LONG
FOR i = 1 TO 5: a(i) = i: NEXT
GOSUB clr: PRINT "1. low above high:";: _ARRAYCOPY a(3 TO 2) TO d(0): GOSUB show
GOSUB clr: PRINT "2. low below LBOUND:";: _ARRAYCOPY a(0 TO 2) TO d(0): GOSUB show
GOSUB clr: PRINT "3. high above UBOUND:";: _ARRAYCOPY a(4 TO 6) TO d(0): GOSUB show
GOSUB clr: PRINT "4. start below LBOUND:";: _ARRAYCOPY a(1 TO 2) TO d(-1): GOSUB show
GOSUB clr: PRINT "5. start above UBOUND:";: _ARRAYCOPY a(1 TO 2) TO d(6): GOSUB show
GOSUB clr: PRINT "6. runs past the end:";: _ARRAYCOPY a(1 TO 3) TO d(4): GOSUB show
GOSUB clr: PRINT "7. whole to start past the end:";: _ARRAYCOPY a() TO d(2): GOSUB show
GOSUB clr: PRINT "8. whole to whole, index 5 fits:";: _ARRAYCOPY a() TO d(): GOSUB show
DIM e(2 TO 4) AS LONG
PRINT "9. whole to whole, destination smaller:";: _ARRAYCOPY a() TO e(): PRINT e(2); e(3); e(4)
PRINT "10. range to whole, outside the destination:";: _ARRAYCOPY a(1 TO 2) TO e(): PRINT e(2); e(3); e(4)
REDIM z(3) AS LONG
ERASE z
GOSUB clr: PRINT "11. erased source:";: _ARRAYCOPY z() TO d(): GOSUB show
PRINT "12. erased destination:";: _ARRAYCOPY a() TO z(): PRINT " done"
'$DYNAMIC
DIM nv(1 TO 5) AS LONG
'$STATIC
ERASE nv
PRINT "13. never allocated again:";: _ARRAYCOPY a(1 TO 2) TO nv(1): PRINT " done"
DIM st(1 TO 5) AS LONG
FOR i = 1 TO 5: st(i) = 9: NEXT
ERASE st
PRINT "14. static after ERASE:";: _ARRAYCOPY a() TO st(): PRINT st(1); st(5)
DIM m(1 TO 2, 1 TO 3) AS LONG, n(1 TO 3, 1 TO 2) AS LONG, k(0 TO 2, 1 TO 3) AS LONG
PRINT "15. 2D whole, other shape:";: _ARRAYCOPY m() TO n(): PRINT " done"
PRINT "16. 2D whole, same counts, other lower:";: _ARRAYCOPY m() TO k(): PRINT " done"
PRINT "17. 2D whole to start, does not fit:";: _ARRAYCOPY m() TO n(2, 1): PRINT " done"
PRINT "18. 2D range outside:";: _ARRAYCOPY m(1 TO 3, 1 TO 1) TO n(1, 1): PRINT " done"
PRINT "19. 2D range to whole outside:";: _ARRAYCOPY m(1 TO 2, 2 TO 3) TO n(): PRINT " done"
DIM s(1 TO 3) AS STRING, t(1 TO 2) AS STRING
s(1) = "a": s(2) = "b": s(3) = "c": t(1) = "keep"
PRINT "20. strings past the end:";: _ARRAYCOPY s() TO t(1): PRINT " "; t(1)
PRINT "21. strings low above high:";: _ARRAYCOPY s(2 TO 1) TO t(1): PRINT " "; t(1)
DIM sm(1 TO 2, 1 TO 2) AS STRING, sn(1 TO 2, 1 TO 3) AS STRING
PRINT "22. 2D strings whole, other bounds:";: _ARRAYCOPY sm() TO sn(): PRINT " done"
PRINT "23. huge range:";: _ARRAYCOPY a(1 TO 9223372036854775807) TO d(0): PRINT " done"
PRINT "24. error in a bound:";: _ARRAYCOPY a(1 TO bad&) TO d(0): GOSUB show
SYSTEM
clr:
FOR i = 0 TO 5: d(i) = -1: NEXT
RETURN
show:
FOR i = 0 TO 5: PRINT d(i);: NEXT
PRINT
RETURN
h:
PRINT " error"; ERR;
RESUME NEXT

FUNCTION bad&
    ERROR 5
    bad& = 2
END FUNCTION
