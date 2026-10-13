$CONSOLE:ONLY
' Verification (step 4 of M3, first task group of m3-console-files): the run-time errors of FIELD, and of GET and
' PUT with no variable: widths beyond the record, a negative width, a file that is not open, a file that is not
' random, what the parts after a failed one do, the forms of the file number and of the width, array elements
' and members as targets. The file is v30_k.tmp and is removed.
ON ERROR GOTO h
TYPE t
    s AS STRING
    n AS LONG
END TYPE
DIM f AS STRING, a AS STRING, b AS STRING, c AS STRING, k AS LONG, r AS t, e(1 TO 3) AS STRING
DIM s AS SINGLE, i AS INTEGER, q(2) AS t
f = "v30_k.tmp"
OPEN f FOR RANDOM AS #1 LEN = 8
PRINT "beyond the record:";: FIELD #1, 20 AS a: PRINT LEN(a)
PRINT "the second part beyond:";: FIELD #1, 5 AS a, 4 AS b, 1 AS c: PRINT LEN(a); LEN(b); LEN(c)
LSET a = "five5": PUT #1, 1
PRINT "exactly the record:";: FIELD #1, 5 AS a, 3 AS b: PRINT LEN(a); LEN(b)
PRINT "a negative width:";: FIELD #1, -1 AS c: PRINT LEN(c)
PRINT "a width that is a fraction:";: FIELD #1, 2.5 AS c: PRINT LEN(c);: FIELD #1, 3.5 AS c: PRINT LEN(c)
s = 2.6: i = 2
PRINT "a width that is an expression:";: FIELD #1, s + i AS c: PRINT LEN(c)
PRINT "no number sign:";: FIELD 1, 4 AS c: PRINT LEN(c)
k = 0
PRINT "a file number that is an expression:";: FIELD #k + 1, 6 AS c: PRINT LEN(c)
PRINT "a file number in parentheses:";: FIELD (1), 7 AS c: PRINT LEN(c)
PRINT "not open:";: FIELD #3, 2 AS c: PRINT LEN(c)
PRINT "file number 0:";: FIELD #0, 2 AS c: PRINT LEN(c)
PRINT "a negative file number:";: FIELD #-1, 2 AS c: PRINT LEN(c)
PRINT "an array element and a member"
FIELD #1, 2 AS e(1), 3 AS e(2), 3 AS r.s
PRINT LEN(e(1)); LEN(e(2)); LEN(e(3)); LEN(r.s)
LSET e(1) = "ab": LSET e(2) = "cde": LSET r.s = "fgh"
FIELD #1, 8 AS a: PRINT "["; a; "]"
k = 2
FIELD #1, 4 AS e(k + 1), 4 AS q(k).s
PRINT LEN(e(3)); LEN(q(2).s); "["; e(3); "]["; q(2).s; "]"
PRINT "an element out of range:";: FIELD #1, 2 AS e(4): PRINT
PRINT "GET and PUT with no variable"
PRINT "GET, record 0:";: GET #1, 0: PRINT SEEK(1)
PRINT "PUT, record -1:";: PUT #1, -1: PRINT SEEK(1)
PRINT "GET #3:";: GET #3: PRINT
PRINT "PUT #3:";: PUT #3: PRINT
CLOSE #1
PRINT "a binary file:";
OPEN f FOR BINARY AS #1
FIELD #1, 2 AS c: PRINT LEN(c);: GET #1: PUT #1: PRINT
CLOSE #1
PRINT "a file open for output:";
OPEN "v30_k2.tmp" FOR OUTPUT AS #1
FIELD #1, 2 AS c: PRINT LEN(c);: GET #1: PUT #1: PRINT
CLOSE #1: KILL "v30_k2.tmp"
PRINT "a file open for input:";
OPEN f FOR INPUT AS #1
FIELD #1, 2 AS c: PRINT LEN(c)
CLOSE #1
PRINT "ACCESS READ:";
OPEN f FOR RANDOM ACCESS READ AS #1 LEN = 8
FIELD #1, 8 AS c: PRINT LEN(c);: GET #1, 1: PRINT "["; c; "]";: PUT #1, 1: PRINT
CLOSE #1
PRINT "ACCESS WRITE:";
OPEN f FOR RANDOM ACCESS WRITE AS #1 LEN = 8
FIELD #1, 8 AS c: PRINT LEN(c);: GET #1, 1: PRINT "["; c; "]";: PUT #1, 1: PRINT
CLOSE #1
KILL f
PRINT "the file number is read before the width"
OPEN f FOR RANDOM AS #1 LEN = 8
FIELD #fn&, wd& AS c: PRINT LEN(c)
FIELD #fn&, wd& AS e(ix&): PRINT LEN(e(1))
CLOSE #1: KILL f
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
FUNCTION fn&
    PRINT "fn ";
    fn& = 1
END FUNCTION
FUNCTION wd&
    PRINT "wd ";
    wd& = 3
END FUNCTION
FUNCTION ix&
    PRINT "ix ";
    ix& = 1
END FUNCTION
