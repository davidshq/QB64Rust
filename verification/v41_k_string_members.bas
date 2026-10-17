$CONSOLE:ONLY
' Verification (m3-arraycopy, while its proposal was written): variable-length STRING member arrays of a plain TYPE
' (no $UNSTABLE) as source and destination, each form; strings are the destination's own afterwards.
TYPE t
    k AS LONG
    s(0 TO 3) AS STRING
END TYPE
DIM i AS LONG
DIM v AS t, w(1 TO 2) AS t, a(1 TO 3) AS STRING
a(1) = "one": a(2) = "two": a(3) = "three"
_ARRAYCOPY a() TO v.s(1)
PRINT "1. array to a member:";: FOR i = 0 TO 3: PRINT " ["; v.s(i); "]";: NEXT: PRINT
a(2) = "changed"
PRINT "2. after the source changed:"; " ["; v.s(2); "]"
_ARRAYCOPY v.s(1 TO 2) TO w(2).s()
PRINT "3. range to whole member:";: FOR i = 0 TO 3: PRINT " ["; w(2).s(i); "]";: NEXT: PRINT
_ARRAYCOPY w(2).s() TO v.s()
PRINT "4. member to member:";: FOR i = 0 TO 3: PRINT " ["; v.s(i); "]";: NEXT: PRINT
_ARRAYCOPY v.s(0 TO 2) TO v.s(1)
PRINT "5. overlap:";: FOR i = 0 TO 3: PRINT " ["; v.s(i); "]";: NEXT: PRINT
PRINT "6. neighbours:"; v.k; w(1).k
SYSTEM
