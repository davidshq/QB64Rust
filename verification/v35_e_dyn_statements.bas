$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): the statements on an element of a _DYNAMIC member: READ,
' SWAP, MID$, INPUT # and LINE INPUT # from a file, PRINT #; and PUT and GET of a whole array whose type has a
' numeric _DYNAMIC member (which bytes go to the file, what GET does to the members).
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
TYPE s
    w(1) _DYNAMIC AS STRING
    f(1) _DYNAMIC AS STRING * 4
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t, z AS s
DIM n AS LONG, i AS LONG
READ x.v(0), x.v(1)
PRINT "1. READ:"; x.v(0); x.v(1)
SWAP x.v(0), x.v(1)
PRINT "2. SWAP:"; x.v(0); x.v(1)
z.w(0) = "hello": MID$(z.w(0), 2, 3) = "EYY": z.f(1) = "abcd": MID$(z.f(1), 1, 1) = "Z"
PRINT "3. MID$: "; z.w(0); " "; z.f(1)
OPEN "v35_e.tmp" FOR OUTPUT AS #1
PRINT #1, 42: PRINT #1, "line two"
CLOSE #1
OPEN "v35_e.tmp" FOR INPUT AS #1
INPUT #1, x.v(1): LINE INPUT #1, z.w(1)
CLOSE #1
PRINT "4. INPUT #, LINE INPUT #:"; x.v(1); " ["; z.w(1); "]"
REDIM p(0 TO 1) AS t
p(0).a = 1: p(0).b = 2: REDIM p(0).v(0 TO 2) AS LONG: p(0).v(0) = &H11111111: p(0).v(1) = &H22222222: p(0).v(2) = &H33333333
p(1).a = 3: p(1).b = 4: p(1).v(1) = &H44444444
OPEN "v35_e.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "5. PUT p() with p(0).v 0 TO 2, declared 0 TO 1 (PUT of x is rejected: v35_x46): LOF ="; n
REDIM _PRESERVE p(0).v(0 TO 1) AS LONG
OPEN "v35_e.bin" FOR BINARY AS #1
PUT #1, 1, p()
n = LOF(1)
CLOSE #1
PRINT "5b. PUT p() with the declared bounds: LOF ="; n
OPEN "v35_e.bin" FOR BINARY AS #1
DIM bytes AS STRING: bytes = SPACE$(n): GET #1, 1, bytes
CLOSE #1
PRINT "   bytes:";
FOR n = 1 TO LEN(bytes): PRINT RIGHT$("0" + HEX$(ASC(bytes, n)), 2);: NEXT: PRINT
REDIM q(0 TO 1) AS t
REDIM q(0).v(5 TO 5) AS LONG: q(0).v(5) = 5
OPEN "v35_e.bin" FOR BINARY AS #1
GET #1, 1, q()
CLOSE #1
GOSUB showq
REDIM q(0).v(0 TO 1) AS LONG
OPEN "v35_e.bin" FOR BINARY AS #1
GET #1, 1, q()
CLOSE #1
PRINT "6b. GET q() after q(0).v has its declared bounds again:"
GOSUB showq
KILL "v35_e.tmp": KILL "v35_e.bin"
SYSTEM
DATA 7, 8
showq:
FOR i = 0 TO 1
    PRINT "6. GET q(): q("; i; "): a, b ="; q(i).a; q(i).b; " v:"; LBOUND(q(i).v); UBOUND(q(i).v);
    FOR n = LBOUND(q(i).v) TO UBOUND(q(i).v): PRINT q(i).v(n);: NEXT: PRINT
NEXT
RETURN
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
