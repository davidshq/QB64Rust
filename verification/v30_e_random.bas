$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): GET and PUT on a file open FOR RANDOM (and with no
' mode). The default record length and LEN =, a record number given and left out, the record after each statement
' (SEEK, LOC), LOF and EOF, a TYPE value, numbers and fixed-length strings shorter than the record, a
' variable-length string (its two length bytes), a value longer than the record, a record past the end and the
' gap before it, record 0 and negative, GET with no variable. The file is v30_e.tmp and is removed.
ON ERROR GOTO h
TYPE rec
    n AS INTEGER
    s AS STRING * 4
    d AS DOUBLE
END TYPE
DIM f AS STRING, r AS rec, r2 AS rec, i AS INTEGER, l AS LONG, v AS STRING, x AS STRING * 6, k AS LONG
DIM ub AS _UNSIGNED _BYTE, c AS STRING * 1, big AS STRING * 20, a(1 TO 3) AS INTEGER
f = "v30_e.tmp"
PRINT "a TYPE value, LEN = LEN(r)"
OPEN f FOR RANDOM AS #1 LEN = LEN(r)
PRINT LEN(r); LOF(1); SEEK(1); LOC(1); EOF(1)
r.n = 1: r.s = "one": r.d = 1.5: PUT #1, 1, r: PRINT LOF(1); SEEK(1); LOC(1)
r.n = 2: r.s = "two": r.d = 2.5: PUT #1, , r: PRINT LOF(1); SEEK(1); LOC(1)
r.n = 3: r.s = "three": r.d = 3.5: PUT #1, 3, r: PRINT LOF(1); SEEK(1); LOC(1)
GET #1, 2, r2: PRINT r2.n; "["; r2.s; "]"; r2.d; SEEK(1); LOC(1); EOF(1)
GET #1, , r2: PRINT r2.n; "["; r2.s; "]"; r2.d; SEEK(1); LOC(1); EOF(1)
GET #1, , r2: PRINT r2.n; "["; r2.s; "]"; r2.d; SEEK(1); LOC(1); EOF(1)
GET #1, 1, r2: PRINT r2.n; "["; r2.s; "]"; r2.d; EOF(1)
r.n = 9: r.s = "nine": r.d = 9.5: PUT #1, 6, r: PRINT LOF(1); SEEK(1)
GET #1, 5, r2: PRINT r2.n; "["; r2.s; "]"; ASC(r2.s); r2.d; EOF(1)
GET #1, 6, r2: PRINT r2.n; "["; r2.s; "]"; r2.d; EOF(1)
GET #1, 7, r2: PRINT r2.n; "["; r2.s; "]"; ASC(r2.s); r2.d; EOF(1); LOF(1)
GET #1, 20, r2: PRINT r2.n; EOF(1); LOF(1); SEEK(1)
PRINT "record 0:";: GET #1, 0, r2: PRINT SEEK(1)
PRINT "record -1:";: PUT #1, -1, r: PRINT SEEK(1)
PRINT "SEEK then GET:";: SEEK #1, 2: GET #1, , r2: PRINT r2.n; SEEK(1)
PRINT "no variable:";: GET #1, 1: PRINT SEEK(1);: GET #1: PRINT SEEK(1);: PUT #1, 9: PRINT SEEK(1); LOF(1)
CLOSE #1
OPEN f FOR RANDOM AS #1 LEN = LEN(r)
PRINT "opened again:"; LOF(1); SEEK(1)
CLOSE #1: KILL f
PRINT "the default length"
OPEN f FOR RANDOM AS #1
i = 258: PUT #1, 1, i: PRINT LOF(1); SEEK(1)
PUT #1, 2, i: PRINT LOF(1); SEEK(1)
i = 0: GET #1, 2, i: PRINT i; SEEK(1); EOF(1)
CLOSE #1: KILL f
OPEN f AS #1
i = 258: PUT #1, 2, i: PRINT LOF(1); SEEK(1)
CLOSE #1: KILL f
PRINT "values shorter than the record, LEN = 8"
OPEN f FOR RANDOM AS #1 LEN = 8
i = 258: PUT #1, 1, i: PRINT LOF(1); SEEK(1)
l = 65537: PUT #1, , l: PRINT LOF(1); SEEK(1)
x = "abcdef": PUT #1, , x: PRINT LOF(1); SEEK(1)
i = 0: GET #1, 1, i: PRINT i; SEEK(1)
l = 0: GET #1, , l: PRINT l; SEEK(1)
x = "": GET #1, , x: PRINT "["; x; "]"; SEEK(1); EOF(1)
PRINT "a LONG read from the INTEGER's record:";: GET #1, 1, l: PRINT l
CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "the bytes:";: FOR k = 1 TO LOF(1): GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
PRINT "a variable-length string, LEN = 8"
OPEN f FOR RANDOM AS #1 LEN = 8
v = "abc": PUT #1, 1, v: PRINT LOF(1); SEEK(1)
v = "abcdef": PUT #1, 2, v: PRINT LOF(1); SEEK(1)
v = "": PUT #1, 3, v: PRINT LOF(1); SEEK(1)
PRINT "seven characters:";: v = "abcdefg": PUT #1, 4, v: PRINT LOF(1); SEEK(1)
v = "?": GET #1, 1, v: PRINT "["; v; "]"; LEN(v); SEEK(1)
v = "?": GET #1, 2, v: PRINT "["; v; "]"; LEN(v)
v = "?": GET #1, 3, v: PRINT "["; v; "]"; LEN(v)
v = "?": GET #1, 9, v: PRINT "["; v; "]"; LEN(v); EOF(1)
CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "the bytes:";: FOR k = 1 TO LOF(1): GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
PRINT "values longer than the record, LEN = 4"
OPEN f FOR RANDOM AS #1 LEN = 4
PRINT "STRING * 6:";: x = "abcdef": PUT #1, 1, x: PRINT LOF(1); SEEK(1)
PRINT "STRING * 20:";: big = "abcdefghijklmnopqrst": PUT #1, 1, big: PRINT LOF(1); SEEK(1)
PRINT "a TYPE of 14:";: PUT #1, 1, r: PRINT LOF(1); SEEK(1)
PRINT "GET a TYPE of 14:";: GET #1, 1, r2: PRINT LOF(1); SEEK(1)
PRINT "an array of 6:";: PUT #1, 1, a(): PRINT LOF(1); SEEK(1)
PRINT "a string of 3:";: v = "abc": PUT #1, 1, v: PRINT LOF(1); SEEK(1)
PRINT "a string of 2:";: v = "ab": PUT #1, 1, v: PRINT LOF(1); SEEK(1)
l = 5: PUT #1, 2, l: PRINT LOF(1); SEEK(1)
CLOSE #1: KILL f
PRINT "LEN ="
PRINT "0:";: OPEN f FOR RANDOM AS #1 LEN = 0: PRINT LOF(1): CLOSE #1
PRINT "-1:";: OPEN f FOR RANDOM AS #1 LEN = -1: PRINT LOF(1): CLOSE #1
PRINT "1:";: OPEN f FOR RANDOM AS #1 LEN = 1: c = "a": PUT #1, 3, c: PRINT LOF(1): CLOSE #1: KILL f
PRINT "2.5:";: OPEN f FOR RANDOM AS #1 LEN = 2.5: i = 1: PUT #1, 2, i: PRINT LOF(1): CLOSE #1: KILL f
PRINT "40000:";: OPEN f FOR RANDOM AS #1 LEN = 40000: i = 1: PUT #1, 2, i: PRINT LOF(1): CLOSE #1: KILL f
PRINT "BINARY LEN = 8:";: OPEN f FOR BINARY AS #1 LEN = 8: i = 1: PUT #1, 2, i: PRINT LOF(1): CLOSE #1: KILL f
PRINT "LOC and SEEK"
OPEN f FOR RANDOM AS #1 LEN = 4
l = 1: PUT #1, 5, l
SEEK #1, 3: PRINT SEEK(1); LOC(1)
SEEK #1, 99: PRINT SEEK(1); LOC(1); EOF(1); LOF(1)
PRINT "SEEK 0:";: SEEK #1, 0: PRINT SEEK(1)
CLOSE #1: KILL f
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
