$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-measured-copies, before its design): PUT and GET of a whole array of a type with a _DYNAMIC
' member array when REDIM gave one element's member another number of dimensions than it is declared with.
' The old compiler's bounds check compares the descriptor's slot 3 with the declared number and then reads the
' bound slots of the declared number (qb64pe.bas 17047).
' 1. one dimension declared (v(1)), REDIMmed to two of the same bytes (0 TO 1, 0 TO 0)
' 2. one declared, two with more bytes (0 TO 1, 0 TO 1)
' 3. two declared (w(1, 1)), REDIMmed to one of the same bytes (0 TO 3)
' 4. two declared, one with fewer bytes (0 TO 1)
' 5. one declared, the same number with other bounds of the same bytes (5 TO 6)
' 6. one declared, the same number with more bytes (0 TO 2)
' Each: PUT (does the file change?), then GET of the file written with the declared bounds (what the member holds).
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE u
    a AS INTEGER
    w(1, 1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG, j AS LONG, n AS LONG
REDIM q(1) AS t
FOR i = 0 TO 1
    q(i).a = 10 + i: q(i).v(0) = 100 + i: q(i).v(1) = 200 + i
NEXT
REDIM r(1) AS u
FOR i = 0 TO 1
    r(i).a = 20 + i: r(i).w(0, 0) = 300 + i: r(i).w(1, 0) = 310 + i: r(i).w(0, 1) = 320 + i: r(i).w(1, 1) = 330 + i
NEXT
OPEN "v38_bt.bin" FOR BINARY AS #1: PUT #1, 1, q(): PRINT "t file:"; LOF(1): CLOSE #1
OPEN "v38_bu.bin" FOR BINARY AS #1: PUT #1, 1, r(): PRINT "u file:"; LOF(1): CLOSE #1

REDIM p(1) AS t
REDIM p(1).v(0 TO 1, 0 TO 0) AS LONG
p(1).v(0, 0) = 7: p(1).v(1, 0) = 8
PRINT "1. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, p(): PRINT LOF(1): CLOSE #1
PRINT "1. GET:";
OPEN "v38_bt.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(0, 0); p(1).v(1, 0); UBOUND(p(1).v, 1); UBOUND(p(1).v, 2)

REDIM p(1) AS t
REDIM p(1).v(0 TO 1, 0 TO 1) AS LONG
p(1).v(0, 0) = 7: p(1).v(1, 0) = 8: p(1).v(0, 1) = 9: p(1).v(1, 1) = 6
PRINT "2. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, p(): PRINT LOF(1): CLOSE #1
PRINT "2. GET:";
OPEN "v38_bt.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(0, 0); p(1).v(1, 0); p(1).v(0, 1); p(1).v(1, 1)

REDIM s(1) AS u
REDIM s(1).w(0 TO 3) AS LONG
FOR j = 0 TO 3: s(1).w(j) = 40 + j: NEXT
PRINT "3. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, s(): PRINT LOF(1): CLOSE #1
PRINT "3. GET:";
OPEN "v38_bu.bin" FOR BINARY AS #1: GET #1, 1, s(): CLOSE #1
PRINT s(0).a; s(0).w(0, 0); s(0).w(1, 1); s(1).a;
FOR j = 0 TO 3: PRINT s(1).w(j);: NEXT
PRINT UBOUND(s(1).w, 1)

REDIM s(1) AS u
REDIM s(1).w(0 TO 1) AS LONG
s(1).w(0) = 50: s(1).w(1) = 51
PRINT "4. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, s(): PRINT LOF(1): CLOSE #1
PRINT "4. GET:";
OPEN "v38_bu.bin" FOR BINARY AS #1: GET #1, 1, s(): CLOSE #1
PRINT s(0).a; s(0).w(0, 0); s(0).w(1, 1); s(1).a; s(1).w(0); s(1).w(1); UBOUND(s(1).w, 1)

REDIM p(1) AS t
REDIM p(1).v(5 TO 6) AS LONG
p(1).v(5) = 7: p(1).v(6) = 8
PRINT "5. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, p(): PRINT LOF(1): CLOSE #1
PRINT "5. GET:";
OPEN "v38_bt.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(5); p(1).v(6); LBOUND(p(1).v)

REDIM p(1) AS t
REDIM p(1).v(0 TO 2) AS LONG
p(1).v(0) = 7: p(1).v(1) = 8: p(1).v(2) = 9
PRINT "6. PUT:";
OPEN "v38_bx.bin" FOR OUTPUT AS #1: CLOSE #1: OPEN "v38_bx.bin" FOR BINARY AS #1: PUT #1, 1, p(): PRINT LOF(1): CLOSE #1
PRINT "6. GET:";
OPEN "v38_bt.bin" FOR BINARY AS #1: GET #1, 1, p(): CLOSE #1
PRINT p(0).a; p(0).v(0); p(0).v(1); p(1).a; p(1).v(0); p(1).v(1); p(1).v(2)

KILL "v38_bt.bin": KILL "v38_bu.bin": KILL "v38_bx.bin"
PRINT "ok"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
