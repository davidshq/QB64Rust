$CONSOLE:ONLY
' Slice (m3-console-files, task 5.3; from verification\v30_e_random): GET and PUT on a file open FOR RANDOM, 12
' numbered lines. Records of a TYPE with LEN = LEN(r): written at a record, at the next one, past the end (a gap
' of zeros), read back in turn and past the end (zeros, EOF); the default record length of 128, also with no mode;
' values shorter than the record (the file ends after them, not at the record's end); a variable-length string
' (two bytes of length and its characters, read back by that length); a value longer than the record (error 59,
' nothing written); record 0 and below (error 63), also for SEEK; GET and PUT with no variable (one whole record
' each). The file is s84_random_file.tmp and is removed.
ON ERROR GOTO handler
TYPE rec
    n AS INTEGER
    s AS STRING * 4
    d AS DOUBLE
END TYPE
DIM f AS STRING, r AS rec, r2 AS rec, i AS INTEGER, l AS LONG, v AS STRING, w AS STRING, x AS STRING * 6, k AS LONG
DIM ub AS _UNSIGNED _BYTE, d AS DOUBLE, big AS STRING * 20
f = "s84_random_file.tmp"
OPEN f FOR RANDOM AS #1 LEN = LEN(r)
PRINT "1:"; LEN(r); LOF(1); SEEK(1); LOC(1); EOF(1)
r.n = 1: r.s = "one": r.d = 1.5: PUT #1, 1, r: PRINT "2:"; LOF(1); SEEK(1); LOC(1);
r.n = 2: r.s = "two": r.d = 2.5: PUT #1, , r: PRINT LOF(1); SEEK(1);
r.n = 9: r.s = "nine": r.d = 9.5: PUT #1, 6, r: PRINT LOF(1); SEEK(1)
PRINT "3:";: GET #1, 2, r2: PRINT r2.n; "["; r2.s; "]"; r2.d; SEEK(1); EOF(1);
GET #1, 1, r2: PRINT r2.n; "["; r2.s; "]";: GET #1, , r2: PRINT r2.n;: GET #1, 6, r2: PRINT r2.n; r2.d; EOF(1)
PRINT "4:";: GET #1, 5, r2: PRINT r2.n; ASC(r2.s); r2.d; EOF(1);: GET #1, 7, r2: PRINT r2.n; EOF(1); LOF(1);
GET #1, 20, r2: PRINT r2.n; EOF(1); LOF(1); SEEK(1)
PRINT "5:";: GET #1, 0, r2: PRINT SEEK(1);: PUT #1, -1, r: PRINT SEEK(1);: SEEK #1, 0: PRINT SEEK(1);: SEEK #1, 2: GET #1, , r2: PRINT r2.n; SEEK(1)
PRINT "6:";: GET #1, 1: PRINT SEEK(1);: GET #1: PRINT SEEK(1);: PUT #1, 9: PRINT SEEK(1); LOF(1)
CLOSE #1: KILL f
OPEN f FOR RANDOM AS #1
i = 258: PUT #1, 1, i: PRINT "7:"; LOF(1); SEEK(1);: PUT #1, 2, i: PRINT LOF(1); SEEK(1);: i = 0: GET #1, 2, i: PRINT i; EOF(1)
CLOSE #1: KILL f
OPEN f AS #1
i = 258: PUT #1, 2, i: PRINT "8:"; LOF(1); SEEK(1)
CLOSE #1: KILL f
OPEN f FOR RANDOM AS #1 LEN = 8
i = 258: PUT #1, 1, i: PRINT "9:"; LOF(1);: l = 65537: PUT #1, , l: PRINT LOF(1);: x = "abcdef": PUT #1, , x: PRINT LOF(1); SEEK(1);
i = 0: GET #1, 1, i: l = 0: GET #1, , l: x = "": GET #1, , x: PRINT i; l; "["; x; "]"; EOF(1);: GET #1, 1, l: PRINT l
v = "abc": PUT #1, 5, v: PRINT "10:"; LOF(1);: w = "": GET #1, 5, w: PRINT "["; w; "]"; LEN(w);
v = "": PUT #1, 6, v: GET #1, 6, w: PRINT LEN(w);: v = "123456": PUT #1, 7, v: w = "keep": GET #1, 7, w: PRINT "["; w; "]"
PRINT "11:";: d = 1: PUT #1, 9, d: PRINT LOF(1);: big = "too long": PUT #1, 9, big: PRINT LOF(1);: GET #1, 1, big: PRINT "["; RTRIM$(big); "]";
v = "1234567": PUT #1, 9, v: PRINT LOF(1)
CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "12:";: FOR k = 1 TO 16: GET #1, k, ub: PRINT ub;: NEXT: FOR k = 33 TO 37: GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
SYSTEM
handler:
PRINT " <error"; ERR; ">";
RESUME NEXT
