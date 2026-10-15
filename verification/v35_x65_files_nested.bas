$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 5.1): PUT and GET of a whole array whose type nests owner types: a member of
' an owner type, an inline member array of one, a _DYNAMIC member whose elements are of an owner type, and one
' of a plain TYPE; a two-dimensional array with a lower bound that is not 0; GET into an array that is not
' allocated; a mismatch inside a nested value.
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
DIM n AS LONG, k AS LONG, i AS LONG, j AS LONG
DIM bytes AS STRING
REDIM p(1 TO 2, 0 TO 0) AS outer
FOR i = 1 TO 2
    p(i, 0).a = i * 100: p(i, 0).b = i
    p(i, 0).i.c = i + 10: p(i, 0).i.w(0) = i * 1000 + 1: p(i, 0).i.w(1) = i * 1000 + 2
    FOR k = 0 TO 1
        p(i, 0).j(k).c = i * 20 + k: p(i, 0).j(k).w(0) = i * 300 + k: p(i, 0).j(k).w(1) = i * 300 + k + 10
    NEXT
    FOR k = 1 TO 2
        p(i, 0).d(k).c = i * 40 + k: p(i, 0).d(k).w(1) = i * 500 + k
    NEXT
    p(i, 0).q(0).x = i * 7: p(i, 0).q(1).y = i * 9
NEXT
PRINT "LEN(pt)"; LEN(p(1, 0).q(0)); " LEN(inner.c)"; LEN(p(1, 0).i.c)
OPEN "v35_x65.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "1. PUT: LOF ="; n
OPEN "v35_x65.bin" FOR BINARY AS #1
bytes = SPACE$(n): GET #1, 1, bytes
CLOSE #1
FOR k = 1 TO LEN(bytes): PRINT RIGHT$("0" + HEX$(ASC(bytes, k)), 2);: NEXT: PRINT
REDIM r(1 TO 2, 0 TO 0) AS outer
OPEN "v35_x65.bin" FOR BINARY AS #1
GET #1, 1, r()
CLOSE #1
PRINT "2. GET:"
GOSUB showr
REDIM r(1 TO 2, 0 TO 0) AS outer
REDIM r(2, 0).d(1).w(0 TO 0) AS INTEGER
r(2, 0).d(1).w(0) = 77
OPEN "v35_x65.bin" FOR BINARY AS #1
GET #1, 1, r()
CLOSE #1
PRINT "3. GET with r(2,0).d(1).w 0 TO 0: r(2,0).d(1).w:"; LBOUND(r(2, 0).d(1).w); UBOUND(r(2, 0).d(1).w); r(2, 0).d(1).w(0)
PRINT "   r(2,0).a, d(1).c, d(2).w(1), q(1).y, b:"; r(2, 0).a; r(2, 0).d(1).c; r(2, 0).d(2).w(1); r(2, 0).q(1).y; r(2, 0).b
REDIM _PRESERVE p(1, 0).j(1).w(0 TO 2) AS INTEGER
OPEN "v35_x65.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "4. PUT with p(1,0).j(1).w 0 TO 2: LOF ="; n
KILL "v35_x65.bin"
SYSTEM
showr:
FOR i = 1 TO 2
    PRINT r(i, 0).a; r(i, 0).b; r(i, 0).i.c; r(i, 0).i.w(0); r(i, 0).i.w(1);
    FOR k = 0 TO 1: PRINT r(i, 0).j(k).c; r(i, 0).j(k).w(0); r(i, 0).j(k).w(1);: NEXT
    FOR k = 1 TO 2: PRINT r(i, 0).d(k).c; r(i, 0).d(k).w(0); r(i, 0).d(k).w(1);: NEXT
    PRINT r(i, 0).q(0).x; r(i, 0).q(0).y; r(i, 0).q(1).x; r(i, 0).q(1).y
NEXT
RETURN
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
