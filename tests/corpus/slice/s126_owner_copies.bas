$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-measured-copies): what the old compiler copies after an error in PUT, GET and a whole-array assignment
' of types with _DYNAMIC member arrays (verification\v38_a, v38_b, v38_d). GET unpacks its own buffer after a
' read that raised an error; a member with other bounds or another number
' of dimensions gets its declared bytes from GET when its block holds them; arrays of such a type with other
' bounds copy the target's number of elements. Left out, as the new compiler differs on purpose: a member block
' with fewer elements (QB64PE-DIVERGENCES.md D-019), a source with fewer elements or not allocated (D-022), a failed
' GET whose buffer was never written (D-021: the old program reads memory nobody wrote, other values each run).
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE u
    a AS INTEGER
    w(1, 1) _DYNAMIC AS LONG
END TYPE
TYPE own
    v(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG, j AS LONG, pass AS LONG

' --- GET after a failed read (v38_a)
REDIM p(1) AS t
REDIM q(1) AS t
FOR i = 0 TO 1
    q(i).a = 10 + i: q(i).v(0) = 100 + i: q(i).v(1) = 200 + i
NEXT
OPEN "s126.bin" FOR BINARY AS #1
PUT #1, 1, q()
CLOSE #1
FOR pass = 1 TO 2
    IF pass = 1 THEN OPEN "s126.bin" FOR BINARY AS #3
    PRINT "2."; pass; ":";
    GOSUB fill
    GET #3, 1, p()
    GOSUB show
    IF pass = 1 THEN CLOSE #3
NEXT
OPEN "s126.bin" FOR BINARY AS #5
PRINT "4. past the end:";
GOSUB fill
GET #5, 15, p()
GOSUB show
CLOSE #5

' --- a member with other bounds or dimensions (v38_b)
REDIM r(1) AS u
FOR i = 0 TO 1
    r(i).a = 20 + i: r(i).w(0, 0) = 300 + i: r(i).w(1, 0) = 310 + i: r(i).w(0, 1) = 320 + i: r(i).w(1, 1) = 330 + i
NEXT
OPEN "s126u.bin" FOR BINARY AS #1: PUT #1, 1, r(): CLOSE #1
REDIM p(1) AS t
REDIM p(1).v(0 TO 1, 0 TO 0) AS LONG
p(1).v(0, 0) = 7: p(1).v(1, 0) = 8
PRINT "5. PUT:";
OPEN "s126x.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "s126x.bin" FOR BINARY AS #1: PUT #1, 1, p(): PRINT LOF(1): CLOSE #1
PRINT "5. GET:";
OPEN "s126.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(0, 0); p(1).v(1, 0); UBOUND(p(1).v, 1); UBOUND(p(1).v, 2)
REDIM p(1) AS t
REDIM p(1).v(0 TO 1, 0 TO 1) AS LONG
p(1).v(0, 1) = 9: p(1).v(1, 1) = 6
PRINT "6. GET:";
OPEN "s126.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(1).a; p(1).v(0, 0); p(1).v(1, 0); p(1).v(0, 1); p(1).v(1, 1)
REDIM s(1) AS u
REDIM s(1).w(0 TO 3) AS LONG
PRINT "7. GET:";
OPEN "s126u.bin" FOR BINARY AS #1: GET #1, 1, s(): CLOSE #1
PRINT s(0).a; s(0).w(0, 0); s(0).w(1, 1); s(1).a;
FOR j = 0 TO 3: PRINT s(1).w(j);: NEXT
PRINT UBOUND(s(1).w, 1)
REDIM p(1) AS t
REDIM p(1).v(5 TO 6) AS LONG
PRINT "8. GET:";
OPEN "s126.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(1).a; p(1).v(5); p(1).v(6); LBOUND(p(1).v)
REDIM p(1) AS t
REDIM p(1).v(0 TO 2) AS LONG
p(1).v(2) = 9
PRINT "9. GET:";
OPEN "s126.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(1).a; p(1).v(0); p(1).v(1); p(1).v(2)
KILL "s126.bin": KILL "s126u.bin": KILL "s126x.bin"

' --- arrays of an owner type with other bounds (v38_d)
DIM so(1 TO 2) AS own
so(1).k = 4: so(2).k = 5: so(1).v(0) = 40: so(2).v(1) = 51
DIM d1(0 TO 1) AS own, d2(0 TO 0) AS own
PRINT "10.";: d1() = so(): PRINT d1(0).k; d1(1).k
PRINT "11.";: d2() = so(): PRINT d2(0).k
REDIM r4(5 TO 6) AS own, r5(0 TO 0) AS own
PRINT "12.";: r4() = so(): PRINT r4(5).k; r4(6).k; LBOUND(r4); UBOUND(r4)
PRINT "13.";: r5() = so(): PRINT r5(0).k
REDIM m(1 TO 4, 0 TO 0) AS own, n(0 TO 1, 0 TO 1) AS own
FOR i = 1 TO 4: m(i, 0).k = 60 + i: NEXT
PRINT "14.";: n() = m(): PRINT n(0, 0).k; n(1, 0).k; n(0, 1).k; n(1, 1).k
PRINT "15.";: d1(0).v(0) = 99: PRINT d1(0).v(0); so(1).v(0); d1(1).v(1)
PRINT "ok"
SYSTEM

fill:
FOR i = 0 TO 1
    p(i).a = -1: p(i).v(0) = -2: p(i).v(1) = -3
NEXT
RETURN

show:
FOR i = 0 TO 1
    PRINT p(i).a; p(i).v(0); p(i).v(1);
NEXT
PRINT
RETURN

h:
PRINT " [error"; ERR; "]";
RESUME NEXT
