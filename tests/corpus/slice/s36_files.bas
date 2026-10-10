$CONSOLE:ONLY
' Sequential files (spec language/file-io): OPEN in both forms, PRINT # with every item kind and separator, WRITE
' to a file and to the console, APPEND, then the file read back with LINE INPUT # and INPUT # into every kind of
' target. Zones in a file are 14 columns; a number is its STR$ and a blank; WRITE puts strings in quotes and numbers
' without blanks; INPUT # reads one field per target. The file is s36_*.tmp and removed again.
DIM f AS STRING, a AS STRING, c AS STRING, k AS LONG, n AS LONG
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, fl AS _FLOAT, o AS _OFFSET
DIM bt AS _BIT * 5, fx AS STRING * 4
DIM arr(3) AS LONG, sa(3) AS STRING
TYPE pt
    x AS LONG
    y AS DOUBLE
    t AS STRING * 3
END TYPE
DIM p AS pt
b = -5: ub = 250: i = -300: ui = 65000: l = -70000: ul = 4000000000: q = -5000000000: uq = 18446744073709551615~&&
s = 1.5: d = -2.25: fl = 1E+20: o = 12: bt = -3: fx = "ab"
f = "s36_files.tmp"

OPEN f FOR OUTPUT AS #1
PRINT #1, "x"; 7; "y"
PRINT #1, 1; -2; 3.5
PRINT #1, "a", "b", 5
PRINT #1, "12345678901234", "z"
PRINT #1, "1234567890123", "z"
PRINT #1, , "after a zone"
PRINT #1, "no newline";
PRINT #1, " same line"
PRINT #1, "zone at the end",
PRINT #1, "next"
PRINT #1,
PRINT #1, ;
PRINT #1, b; ub; i; ui; l; ul; q; uq
PRINT #1, s; d; fl; o; bt; "["; fx; "]"
PRINT #1, LEFT$("left", 2) + "!"; LEN(a) + 1
WRITE #1, 1, "a", 2.5
WRITE #1, b, ub, i, ui, l, ul, q, uq
WRITE #1, s, d, fl, o, bt, fx
WRITE #1, -1, -2.5, 1E+30, .5, -.5, 100000000
WRITE #1, "with " + CHR$(34) + "quote", "comma, inside", ""
WRITE #1,
WRITE #1, "open",
WRITE #1, "closed"
CLOSE #1

n = 2
OPEN f FOR APPEND AS n
PRINT #n, "appended through a variable"
PRINT #1.5, "and through 1.5, which is file 2"
CLOSE n

OPEN "A", #3, f
PRINT #3, "old form A"
CLOSE

OPEN f FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, a
    k = k + 1
    PRINT k; "["; a; "]"
LOOP
CLOSE #1

PRINT "console WRITE"
WRITE 1, "a", 2.5
WRITE b, s, fx
WRITE
WRITE "open",
WRITE "closed"

PRINT "INPUT # into every type"
OPEN f FOR OUTPUT AS #1
WRITE #1, 1, 2, 3, 4, 5, 6, 7, 8
WRITE #1, 1.5, 2.5, 3.5, 12, -3
WRITE #1, "plain", "with, comma", 42
PRINT #1, "  lead , trail  ,7"
PRINT #1, "2.5,3.5,&H10,12x,,"
PRINT #1, "9"
PRINT #1, "10"
PRINT #1, "s1,s2,21,22,23.5,xyzzy"
PRINT #1, "a line, with a comma"
PRINT #1, "fixed length"
PRINT #1, "last line without an end";
CLOSE #1
OPEN "I", 1, f
INPUT #1, b, ub, i, ui, l, ul, q, uq
PRINT b; ub; i; ui; l; ul; q; uq
INPUT #1, s, d, fl, o, bt
PRINT s; d; fl; o; bt
INPUT #1, a, c, l
PRINT "["; a; "]["; c; "]"; l
INPUT #1, a, c, l
PRINT "["; a; "]["; c; "]"; l
INPUT #1, i, l, ui, n, s, a
PRINT i; l; ui; n; s; "["; a; "]"
INPUT #1, i, l
PRINT i; l
INPUT #1, sa(1), fx, arr(2), p.x, p.y, p.t
PRINT "["; sa(1); "]["; fx; "]"; arr(2); p.x; p.y; "["; p.t; "]"
LINE INPUT #1, sa(2)
PRINT "["; sa(2); "]"
LINE INPUT #1, fx
PRINT "["; fx; "]"; EOF(1)
LINE INPUT #1, a
PRINT "["; a; "]"; EOF(1)
CLOSE
KILL f
SYSTEM
