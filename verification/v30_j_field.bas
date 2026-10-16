$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): FIELD on a file open FOR RANDOM. Records
' written through fields and read back, GET and PUT with no variable, what a field variable holds after FIELD,
' a second FIELD on the same file, a variable named by two FIELDs, = to a field variable, GET and PUT with a
' variable beside fields, a field variable after CLOSE. The file is v30_j.tmp and is removed.
ON ERROR GOTO h
DIM f AS STRING, a AS STRING, b AS STRING, c AS STRING, d AS STRING, w AS STRING, k AS LONG
DIM ub AS _UNSIGNED _BYTE, l AS LONG, x AS STRING * 8, n AS INTEGER
f = "v30_j.tmp"
PRINT "a record through fields, LEN = 8"
OPEN f FOR RANDOM AS #1 LEN = 8
a = "before"
PRINT "["; a; "]"; LEN(a); LEN(b)
FIELD #1, 3 AS a, 5 AS b
PRINT "after FIELD:"; LEN(a); LEN(b); ASC(a); ASC(b)
LSET a = "xy": LSET b = "hello"
PRINT "["; a; "]["; b; "]"
PUT #1, 1: PRINT LOF(1); SEEK(1)
LSET a = "": LSET b = ""
PRINT "["; a; "]["; b; "]"
GET #1, 1: PRINT "["; a; "]["; b; "]"; SEEK(1)
RSET a = "z": RSET b = "ab"
PRINT "["; a; "]["; b; "]"
PUT #1: PRINT LOF(1); SEEK(1)
PUT #1, 4: PRINT LOF(1); SEEK(1)
GET #1, 2: PRINT "["; a; "]["; b; "]"
GET #1, 3: PRINT "["; a; "]"; ASC(a); ASC(b); EOF(1)
GET #1, 9: PRINT ASC(a); ASC(b); EOF(1); LOF(1)
PRINT "a second FIELD on the file"
GET #1, 1
FIELD #1, 8 AS w
PRINT "["; w; "]["; a; "]["; b; "]"
LSET w = "12345678"
PRINT "["; w; "]["; a; "]["; b; "]"
LSET b = "B"
PRINT "["; w; "]["; a; "]["; b; "]"
FIELD #1, 2 AS c, 2 AS d
PRINT "["; c; "]["; d; "]["; w; "]"
PRINT "a variable named again"
FIELD #1, 4 AS a
PRINT "["; a; "]"; LEN(a)
LSET w = "abcdefgh"
PRINT "["; a; "]["; b; "]["; c; "]"
PRINT "= to a field variable"
a = "QQ"
PRINT "["; a; "]"; LEN(a); "["; w; "]"
LSET w = "ABCDEFGH"
PRINT "["; a; "]"; LEN(a); "["; w; "]"
a = "longer than the field"
PRINT "["; a; "]"; LEN(a)
GET #1, 1
PRINT "["; a; "]"; LEN(a); "["; w; "]"
b = "": PRINT LEN(b);: LSET b = "k": PRINT LEN(b); "["; b; "]["; w; "]"
PRINT "GET and PUT with a variable beside fields"
LSET w = "ABCDEFGH": PUT #1, 1
l = 0: GET #1, 1, l: PRINT l; "["; w; "]"
x = "ijklmnop": PUT #1, 1, x: PRINT "["; w; "]"
GET #1, 1: PRINT "["; w; "]"
PRINT "a width of 0, and all of the record"
FIELD #1, 0 AS c, 8 AS d
PRINT LEN(c); "["; d; "]"
LSET c = "x": PRINT LEN(c)
PRINT "after CLOSE"
CLOSE #1
PRINT "["; w; "]"; LEN(w)
LSET w = "closed": PRINT "["; w; "]"
w = "free": PRINT "["; w; "]"
OPEN f FOR RANDOM AS #1 LEN = 8
PRINT "opened again:"; LOF(1); "["; d; "]"
LSET d = "again": PRINT "["; d; "]"
PUT #1, 1
FIELD #1, 8 AS w
PRINT "["; w; "]"
GET #1, 1: PRINT "["; w; "]["; d; "]"
CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "the bytes:";: FOR k = 1 TO LOF(1): GET #1, k, ub: PRINT ub;: NEXT: PRINT
CLOSE #1: KILL f
PRINT "the default length"
OPEN f FOR RANDOM AS #1
FIELD #1, 100 AS a, 28 AS b
PRINT LEN(a); LEN(b)
FIELD #1, 100 AS a, 29 AS b
PRINT LEN(a); LEN(b)
CLOSE #1: KILL f
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
