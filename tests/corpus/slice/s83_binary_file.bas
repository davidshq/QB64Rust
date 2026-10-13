$CONSOLE:ONLY
' Slice (m3-console-files, task 5.3; from verification\v30_d_binary, v30_x39, x43-x46): GET and PUT on a file open
' FOR BINARY, 16 numbered lines. The size of each numeric type in the file and the values read back; the position
' given, left out and as a fraction (SEEK, LOC); a variable-length string (as many characters as it holds, an
' empty one reads nothing) and a fixed-length one; a TYPE variable, a member, an element, an element of a string
' array; a whole array of numbers, of fixed-length strings, of a TYPE, a dynamic one and one of two dimensions; a
' GET at and past the end (zeros, EOF, the file not grown); a PUT past the end (the gap is 0); the errors: position
' 0 and below (63), no variable (54), a file not open (52), a sequential file (54), ACCESS READ and WRITE (75);
' after CALL; an index outside (error 9, element 0 stands in). The file is s83_binary_file.tmp and is removed.
ON ERROR GOTO handler
TYPE rec
    n AS LONG
    s AS STRING * 6
    d AS DOUBLE
END TYPE
DIM f AS STRING, b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE
DIM d AS DOUBLE, fl AS _FLOAT, o AS _OFFSET, v AS STRING, x AS STRING * 4, r AS rec, r2 AS rec, k AS LONG
DIM a(1 TO 4) AS LONG, a2(1 TO 4) AS LONG, e(1 TO 2) AS STRING, fa(1 TO 2) AS STRING * 3, ra(1 TO 2) AS rec
DIM t2(1 TO 2, 1 TO 3) AS INTEGER, n AS LONG
f = "s83_binary_file.tmp"
OPEN f FOR BINARY AS #1
i = 258: l = 65537: d = 1.5: b = -2
PUT #1, , i: PUT #1, , l: PUT #1, , d: PUT #1, , b
PRINT "1:"; LOF(1); SEEK(1); LOC(1)
i = 0: l = 0: d = 0: b = 0
GET #1, 1, i: GET #1, , l: GET #1, , d: GET #1, , b
PRINT "2:"; i; l; d; b; SEEK(1); EOF(1)
q = 9: s = 2.5: fl = 3.5: o = 7
PUT #1, 1, q: PRINT "3:"; SEEK(1);: PUT #1, 1, s: PRINT SEEK(1);: PUT #1, 1, fl: PRINT SEEK(1);: PUT #1, 1, o: PRINT SEEK(1)
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
v = "abcdef": PUT #1, 1, v
PRINT "4:"; LOF(1);: v = "": GET #1, 1, v: PRINT "["; v; "]"; SEEK(1);: v = SPACE$(3): GET #1, 1, v: PRINT "["; v; "]"; SEEK(1);
v = SPACE$(9): GET #1, 2, v: PRINT "["; RTRIM$(v); "]"; ASC(v, 6); EOF(1); LOF(1)
x = "wxyz": PUT #1, 3, x: x = "": GET #1, 1, x
PRINT "5: ["; x; "]";: GET #1, 2.5, ub: PRINT ub;: GET #1, 3.5, ub: PRINT ub; SEEK(1)
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
r.n = 7: r.s = "abc": r.d = 2.25
PUT #1, 1, r: PRINT "6:"; LOF(1);: GET #1, 1, r2: PRINT r2.n; "["; r2.s; "]"; r2.d;
r.n = 99: PUT #1, 1, r.n: r.s = "member": PUT #1, 5, r.s: GET #1, 1, r2: PRINT r2.n; "["; r2.s; "]"; LOF(1)
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
a(1) = 11: a(2) = 22: a(3) = 33: a(4) = 44
PUT #1, 1, a(): PRINT "7:"; LOF(1);: GET #1, 1, a2(): PRINT a2(1); a2(2); a2(3); a2(4);
k = 3: PUT #1, 1, a(k): GET #1, 1, l: PRINT l;: GET #1, 5, a2(k + 1): PRINT a2(4)
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
e(1) = "ab": e(2) = "cde"
PUT #1, 1, e(2): PRINT "8:"; LOF(1);: v = "??": GET #1, 1, v: PRINT v;: GET #1, 2, e(1): PRINT " "; e(1)
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
fa(1) = "ab": fa(2) = "cde"
PUT #1, 1, fa(): PRINT "9:"; LOF(1);: fa(1) = "": GET #1, 4, fa(1): PRINT "["; fa(1); "]";
ra(1).n = 1: ra(2).n = 2: ra(2).s = "two"
PUT #1, 1, ra(): PRINT LOF(1);: GET #1, 19, r2: PRINT r2.n; "["; r2.s; "]";: PUT #1, 1, ra(2): GET #1, 1, l: PRINT l
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
n = 5
DIM da(1 TO n) AS LONG
da(5) = 55: PUT #1, 1, da(): PRINT "10:"; LOF(1);: REDIM da(1 TO 2) AS LONG: GET #1, 17, da(): PRINT da(1); SEEK(1);
t2(2, 3) = 6: PUT #1, 1, t2(): GET #1, 11, i: PRINT i
CLOSE #1: KILL f
OPEN f FOR BINARY AS #1
l = 16909060: PUT #1, 1, l
l = -1: GET #1, 3, l: PRINT "11:"; l; EOF(1); LOF(1); SEEK(1);: l = -1: GET #1, 5, l: PRINT l; EOF(1); LOF(1);
l = -1: GET #1, 20, l: PRINT l; EOF(1); LOF(1); SEEK(1)
ub = 9: PUT #1, 10, ub: PRINT "12:"; LOF(1);: FOR k = 5 TO 10: GET #1, k, ub: PRINT ub;: NEXT: PRINT
PRINT "13:";: PUT #1, 0, ub: PRINT SEEK(1);: GET #1, -1, ub: PRINT SEEK(1);: GET #1: PUT #1, 2: PRINT LOF(1)
PRINT "14:";: GET #3, , ub: PUT #3, 1, ub: PRINT
CALL PUT(#1, 1, i): CALL GET(1, 1, l)
PRINT "15:"; SEEK(1);: PUT #1, 1, a(9): PRINT SEEK(1);: GET #1, 1, a2(0): PRINT SEEK(1)
CLOSE #1
OPEN f FOR OUTPUT AS #1: PRINT "16:";: PUT #1, , ub: GET #1, , ub: CLOSE #1
OPEN f FOR INPUT AS #1: GET #1, , ub: CLOSE #1
OPEN f FOR BINARY ACCESS READ AS #1: PUT #1, 1, ub: GET #1, 1, ub: CLOSE #1
OPEN f FOR BINARY ACCESS WRITE AS #1: GET #1, 1, ub: PUT #1, 1, ub: CLOSE #1
PRINT
KILL f
SYSTEM
handler:
PRINT " <error"; ERR; ">";
RESUME NEXT
