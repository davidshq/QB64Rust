$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): whole-array assignment with a member array, without
' $UNSTABLE:TYPEFIELDS: member to member, member to array, array to member, members of array elements, strings,
' a member array of a TYPE, bounds of a member that differ from the array's.
TYPE leaf
    k AS LONG
    s AS STRING
END TYPE
TYPE box
    v(1 TO 3) AS LONG
    w(1) AS STRING
    l(1) AS leaf
END TYPE
DIM x AS box, y AS box, i AS LONG
DIM a(1 TO 3) AS LONG
DIM b(0 TO 2) AS LONG
DIM e(2) AS box
ON ERROR GOTO h
FOR i = 1 TO 3: x.v(i) = i * 5: NEXT
x.w(0) = "w0": x.w(1) = "w1": x.l(1).k = 9: x.l(1).s = "leaf"
PRINT "1. member to member:";
y.v() = x.v(): y.w() = x.w(): y.l() = x.l()
x.w(1) = "changed": x.l(1).s = "changed"
PRINT y.v(1); y.v(3); " "; y.w(0); " "; y.w(1); y.l(1).k; " "; y.l(1).s
PRINT "2. member to array:";
a() = x.v()
FOR i = 1 TO 3: PRINT a(i);: NEXT: PRINT
PRINT "3. array to member:";
a(2) = -1
x.v() = a()
FOR i = 1 TO 3: PRINT x.v(i);: NEXT: PRINT
PRINT "4. members of array elements:";
e(1).v(2) = 77: e(1).w(1) = "e1"
e(2).v() = e(1).v(): e(2).w() = e(1).w()
PRINT e(2).v(2); " "; e(2).w(1)
PRINT "5. other bounds (error):";
b() = x.v()
PRINT " after:"; b(0)
PRINT "6. an element of a member and a whole element:";
e(0) = e(1)
PRINT e(0).v(2)
SYSTEM
h:
PRINT " error"; ERR;
RESUME NEXT
