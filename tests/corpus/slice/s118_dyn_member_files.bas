$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-typefields task 5.3): PUT and GET of a whole array whose type has _DYNAMIC member arrays
' (verification\v35_e 5.-6b, v35_x55, v35_x65): the declared layout in the file, a member with other bounds (PUT
' raises error 9 and writes nothing), a file open FOR RANDOM (the whole array is one record), nested owner types,
' a two-dimensional array with a lower bound that is not 0. Where the new compiler differs on purpose
' (QB64PE-DIVERGENCES.md D-019: GET into a member with other bounds raises error 9 and keeps the member as it was; the old
' compiler writes the declared bytes into its smaller block) the lines were recorded from a program that, in place
' of that GET, gives the member its declared bounds, reads, raises ERROR 9 and puts the member's bounds and element
' back.
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
TYPE r
    n AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE pt
    x AS INTEGER
    y AS INTEGER
END TYPE
TYPE inner
    c AS _BYTE
    w(0 TO 1) _DYNAMIC AS INTEGER
END TYPE
TYPE outer
    a AS INTEGER
    i AS inner
    j(1) AS inner
    d(1 TO 2) _DYNAMIC AS inner
    q(1) _DYNAMIC AS pt
    b AS _BYTE
END TYPE
ON ERROR GOTO h
DIM n AS LONG, i AS LONG, k AS LONG
DIM bytes AS STRING
REDIM p(0 TO 1) AS t
p(0).a = 1: p(0).b = 2: REDIM p(0).v(0 TO 2) AS LONG: p(0).v(0) = &H11111111: p(0).v(1) = &H22222222: p(0).v(2) = &H33333333
p(1).a = 3: p(1).b = 4: p(1).v(1) = &H44444444
OPEN "s118.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "1. PUT p() with p(0).v 0 TO 2, declared 0 TO 1: LOF ="; n
REDIM _PRESERVE p(0).v(0 TO 1) AS LONG
OPEN "s118.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "2. PUT p() with the declared bounds: LOF ="; n
OPEN "s118.bin" FOR BINARY AS #1
bytes = SPACE$(n): GET #1, 1, bytes
CLOSE #1
GOSUB showbytes
REDIM q(0 TO 1) AS t
REDIM q(0).v(5 TO 5) AS LONG: q(0).v(5) = 5
OPEN "s118.bin" FOR BINARY AS #1
GET #1, 1, q()
CLOSE #1
PRINT "3. GET q() with q(0).v 5 TO 5:"
GOSUB showq
REDIM q(0).v(0 TO 1) AS LONG
OPEN "s118.bin" FOR BINARY AS #1
GET #1, 1, q()
CLOSE #1
PRINT "4. GET q() after q(0).v has its declared bounds again:"
GOSUB showq
KILL "s118.bin"
REDIM rr(1) AS r
rr(1).n = 5: rr(1).v(1) = 7
OPEN "s118.tmp" FOR RANDOM AS #1 LEN = 10
PUT #1, 1, rr()
PRINT "5. RANDOM, LEN = 10:"; LOF(1)
CLOSE #1
OPEN "s118.tmp" FOR RANDOM AS #1 LEN = 20
PUT #1, 1, rr()
PRINT "6. RANDOM, LEN = 20:"; LOF(1)
CLOSE #1
REDIM rr(1) AS r
OPEN "s118.tmp" FOR RANDOM AS #1 LEN = 20
GET #1, 1, rr()
CLOSE #1
PRINT "7. GET from it:"; rr(1).n; rr(1).v(0); rr(1).v(1)
KILL "s118.tmp"
REDIM o(1 TO 2, 0 TO 0) AS outer
FOR i = 1 TO 2
    o(i, 0).a = i * 100: o(i, 0).b = i
    o(i, 0).i.c = i + 10: o(i, 0).i.w(0) = i * 1000 + 1: o(i, 0).i.w(1) = i * 1000 + 2
    FOR k = 0 TO 1
        o(i, 0).j(k).c = i * 20 + k: o(i, 0).j(k).w(0) = i * 300 + k: o(i, 0).j(k).w(1) = i * 300 + k + 10
    NEXT
    FOR k = 1 TO 2
        o(i, 0).d(k).c = i * 40 + k: o(i, 0).d(k).w(1) = i * 500 + k
    NEXT
    o(i, 0).q(0).x = i * 7: o(i, 0).q(1).y = i * 9
NEXT
OPEN "s118.bin" FOR BINARY AS #1
PUT #1, 1, o()
n = LOF(1)
CLOSE #1
PRINT "8. PUT o() of nested owner types: LOF ="; n
OPEN "s118.bin" FOR BINARY AS #1
bytes = SPACE$(n): GET #1, 1, bytes
CLOSE #1
GOSUB showbytes
REDIM g(1 TO 2, 0 TO 0) AS outer
OPEN "s118.bin" FOR BINARY AS #1
GET #1, 1, g()
CLOSE #1
PRINT "9. GET g():"
GOSUB showg
REDIM g(1 TO 2, 0 TO 0) AS outer
REDIM g(2, 0).d(1).w(0 TO 0) AS INTEGER
g(2, 0).d(1).w(0) = 77
OPEN "s118.bin" FOR BINARY AS #1
GET #1, 1, g()
CLOSE #1
PRINT "10. GET with g(2,0).d(1).w 0 TO 0:"; LBOUND(g(2, 0).d(1).w); UBOUND(g(2, 0).d(1).w); g(2, 0).d(1).w(0)
PRINT "    g(2,0).a, d(1).c, d(2).w(1), q(1).y, b:"; g(2, 0).a; g(2, 0).d(1).c; g(2, 0).d(2).w(1); g(2, 0).q(1).y; g(2, 0).b
KILL "s118.bin"
REDIM _PRESERVE o(1, 0).j(1).w(0 TO 2) AS INTEGER
OPEN "s118.bin" FOR BINARY AS #1
PUT #1, 1, o()
n = LOF(1)
CLOSE #1
PRINT "11. PUT with o(1,0).j(1).w 0 TO 2: LOF ="; n
KILL "s118.bin"
SYSTEM
showbytes:
PRINT "   bytes:";
FOR k = 1 TO LEN(bytes): PRINT RIGHT$("0" + HEX$(ASC(bytes, k)), 2);: NEXT: PRINT
RETURN
showq:
FOR i = 0 TO 1
    PRINT "   q("; i; "): a, b ="; q(i).a; q(i).b; " v:"; LBOUND(q(i).v); UBOUND(q(i).v);
    FOR n = LBOUND(q(i).v) TO UBOUND(q(i).v): PRINT q(i).v(n);: NEXT: PRINT
NEXT
RETURN
showg:
FOR i = 1 TO 2
    PRINT g(i, 0).a; g(i, 0).b; g(i, 0).i.c; g(i, 0).i.w(0); g(i, 0).i.w(1);
    FOR k = 0 TO 1: PRINT g(i, 0).j(k).c; g(i, 0).j(k).w(0); g(i, 0).j(k).w(1);: NEXT
    FOR k = 1 TO 2: PRINT g(i, 0).d(k).c; g(i, 0).d(k).w(0); g(i, 0).d(k).w(1);: NEXT
    PRINT g(i, 0).q(0).x; g(i, 0).q(0).y; g(i, 0).q(1).x; g(i, 0).q(1).y
NEXT
RETURN
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
