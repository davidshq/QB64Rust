$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): GET and PUT on a file open FOR BINARY. Each numeric
' type and its size in the file, a variable-length and a fixed-length string, a TYPE value, an array element, a
' whole array, a position given and left out, the position after each statement (SEEK, LOC), a GET at and past
' the end (EOF, what the variable holds), a PUT past the end, positions 0 and negative, forms with and without #,
' with no variable, and a file number that is not open or open in another mode. The file is v30_d.tmp, removed.
ON ERROR GOTO h
TYPE rec
    n AS INTEGER
    s AS STRING * 4
    d AS DOUBLE
END TYPE
DIM f AS STRING, i AS INTEGER, l AS LONG, s AS SINGLE, d AS DOUBLE, q AS _INTEGER64, b AS _BYTE
DIM ub AS _UNSIGNED _BYTE, fl AS _FLOAT, o AS _OFFSET
DIM v AS STRING, x AS STRING * 6, r AS rec, r2 AS rec, a(1 TO 4) AS INTEGER, a2(1 TO 4) AS INTEGER, k AS LONG
DIM c AS STRING * 1
f = "v30_d.tmp"
PRINT "sizes"
OPEN f FOR BINARY AS #1
i = 258: PUT #1, , i: PRINT LOF(1); SEEK(1); LOC(1)
l = 65536: PUT #1, , l: PRINT LOF(1); SEEK(1); LOC(1)
s = 1.5: PUT #1, , s: PRINT LOF(1);
d = 2.5: PUT #1, , d: PRINT LOF(1);
q = -2: PUT #1, , q: PRINT LOF(1);
b = -1: PUT #1, , b: PRINT LOF(1);
ub = 200: PUT #1, , ub: PRINT LOF(1);
fl = 3.5: PUT #1, , fl: PRINT LOF(1);
o = 7: PUT #1, , o: PRINT LOF(1)
i = 0: l = 0: s = 0: d = 0: q = 0: b = 0: ub = 0: fl = 0: o = 0
GET #1, 1, i: GET #1, , l: GET #1, , s: GET #1, , d: GET #1, , q: GET #1, , b: GET #1, , ub: GET #1, , fl
GET #1, , o
PRINT i; l; s; d; q; b; ub; fl; o
PRINT "bytes of 258 and 65536:";
FOR k = 1 TO 6: GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
PRINT "strings"
OPEN f FOR BINARY AS #1
v = "hello": PUT #1, , v: PRINT LOF(1); SEEK(1)
x = "ab": PUT #1, , x: PRINT LOF(1); SEEK(1)
v = "": PUT #1, , v: PRINT LOF(1); SEEK(1)
v = "???": GET #1, 1, v: PRINT "["; v; "]"; SEEK(1)
v = SPACE$(20): GET #1, 1, v: PRINT "["; v; "]"; LEN(v); SEEK(1); EOF(1)
v = "": GET #1, 1, v: PRINT "["; v; "]"; SEEK(1)
x = "zzzzzz": GET #1, 8, x: PRINT "["; x; "]"; ASC(x, 5); SEEK(1); EOF(1)
c = "z": GET #1, 2, c: PRINT "["; c; "]"
CLOSE #1: KILL f
PRINT "a TYPE value, elements, a whole array"
OPEN f FOR BINARY AS #1
r.n = 5: r.s = "wxyz": r.d = 1.25: PUT #1, , r: PRINT LOF(1); LEN(r)
GET #1, 1, r2: PRINT r2.n; "["; r2.s; "]"; r2.d
PUT #1, 1, r.n: PUT #1, , r.s: PRINT SEEK(1)
r2.n = 0: r2.s = "": GET #1, 1, r2.n: GET #1, , r2.s: PRINT r2.n; "["; r2.s; "]"
FOR k = 1 TO 4: a(k) = k * 11: NEXT
PUT #1, 1, a(): PRINT SEEK(1)
GET #1, 1, a2(): PRINT a2(1); a2(2); a2(3); a2(4); SEEK(1)
PUT #1, 1, a(3): PRINT SEEK(1)
GET #1, 1, a2(1): PRINT a2(1)
CLOSE #1: KILL f
PRINT "positions and the end"
OPEN f FOR BINARY AS #1
v = "abcdef": PUT #1, , v
PRINT SEEK(1); LOC(1); EOF(1)
i = 77: GET #1, , i: PRINT i; SEEK(1); EOF(1); LOF(1)
i = 77: GET #1, 6, i: PRINT i; SEEK(1); EOF(1)
i = 77: GET #1, 5, i: PRINT i; SEEK(1); EOF(1)
i = 77: GET #1, 100, i: PRINT i; SEEK(1); EOF(1); LOF(1)
GET #1, 1, c: PRINT c; EOF(1)
PUT #1, 10, c: PRINT LOF(1); SEEK(1)
FOR k = 7 TO 10: GET #1, k, ub: PRINT ub;: NEXT: PRINT
PRINT "position 0:";: PUT #1, 0, c: PRINT SEEK(1)
PRINT "position -1:";: PUT #1, -1, c: PRINT SEEK(1)
PRINT "GET position 0:";: GET #1, 0, c: PRINT SEEK(1)
PRINT "position 2.5:";: GET #1, 2.5, c: PRINT c; SEEK(1)
PRINT "position 3.5:";: GET #1, 3.5, c: PRINT c; SEEK(1)
l = 3: GET #1, l, c: PRINT c; SEEK(1)
q = 4: GET #1, q + 1, c: PRINT c; SEEK(1)
PRINT "SEEK then GET:";: SEEK #1, 2: GET #1, , c: PRINT c; SEEK(1)
CLOSE #1: KILL f
PRINT "forms"
OPEN f FOR BINARY AS #1
c = "m": PUT 1, , c: PUT #1, 2, c: PUT 1, 3, c: PRINT LOF(1)
c = "": GET 1, 1, c: PRINT c;: c = "": GET 1, , c: PRINT c
k = 1: GET #k, 1, c: PRINT c: GET k, 1, c: PRINT c
GET #1.4, 1, c: PRINT c
PRINT "no variable:";: GET #1: PRINT SEEK(1);: GET #1, 2: PRINT SEEK(1);: PUT #1: PRINT SEEK(1);: PUT #1, 3: PRINT SEEK(1); LOF(1)
CLOSE #1: KILL f
PRINT "other modes"
PRINT "not open:";: GET #3, , c: PRINT
PRINT "not open:";: PUT #3, , c: PRINT
PRINT "number 0:";: GET #0, , c: PRINT
PRINT "number -1:";: GET #-1, , c: PRINT
OPEN f FOR OUTPUT AS #1: PRINT #1, "line"
PRINT "OUTPUT:";: PUT #1, , c: GET #1, , c: PRINT
CLOSE #1
OPEN f FOR INPUT AS #1
PRINT "INPUT:";: PUT #1, , c: GET #1, , c: PRINT
CLOSE #1
OPEN f FOR APPEND AS #1
PRINT "APPEND:";: PUT #1, , c: GET #1, , c: PRINT
CLOSE #1
OPEN f FOR BINARY ACCESS READ AS #1
PRINT "ACCESS READ:";: PUT #1, , c: PRINT ;: GET #1, 1, c: PRINT c
CLOSE #1
OPEN f FOR BINARY ACCESS WRITE AS #1
PRINT "ACCESS WRITE:";: GET #1, , c: PRINT ;: PUT #1, 1, c: PRINT LOF(1)
CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "PRINT # to BINARY:";: PRINT #1, "ab": PRINT LOF(1)
PRINT "INPUT # from BINARY:";: SEEK #1, 1: LINE INPUT #1, v: PRINT "["; v; "]"
CLOSE #1: KILL f
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
