$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Files"): PRINT # with every item kind, zones in a file, trailing ; and ,,
' the empty forms; WRITE to a file and to the console of every numeric type, negative numbers, floats, strings with
' quotes, no items, a trailing comma; a raising item; a number that is not open. The file is read back line by line.
ON ERROR GOTO h
DIM f AS STRING, a AS STRING, k AS LONG
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, fl AS _FLOAT, o AS _OFFSET
DIM bt AS _BIT * 5, fx AS STRING * 4
b = -5: ub = 250: i = -300: ui = 65000: l = -70000: ul = 4000000000: q = -5000000000: uq = 18446744073709551615
s = 1.5: d = -2.25: fl = 1E+20: o = 12: bt = -3: fx = "ab"
f = "v22_b_p.tmp"
OPEN f FOR OUTPUT AS #1
PRINT #1, "x"; 7; "y"
PRINT #1, 1; -2; 3.5
PRINT #1, "a", "b", 5
PRINT #1, "12345678901234", "z"
PRINT #1, "1234567890123", "z"
PRINT #1, "no newline";
PRINT #1, " same line"
PRINT #1, "zone at the end",
PRINT #1, "next"
PRINT #1,
PRINT #1, ;
PRINT #1, "after two empties"
PRINT #1, b; ub; i; ui; l; ul; q; uq
PRINT #1, s; d; fl; o; bt; "["; fx; "]"
PRINT #1, "a"; "b"; 1; 2
PRINT #1, "ra"; CHR$(-1); "rb"
PRINT #1, "after raise"
PRINT #1, TAB(5); "t"; SPC(3); "s"
WRITE #1, 1, "a", 2.5
WRITE #1, b, ub, i, ui, l, ul, q, uq
WRITE #1, s, d, fl, o, bt, fx
WRITE #1, -1, -2.5, 1E+30, .5, -.5, 100000000
WRITE #1, "with " + CHR$(34) + "quote", "comma, inside", ""
WRITE #1,
WRITE #1, "ra", CHR$(-1), "rb"
WRITE #1, "after raise"
CLOSE #1
OPEN f FOR INPUT AS #1
DO UNTIL EOF(1)
    LINE INPUT #1, a
    k = k + 1
    PRINT k; "["; a; "]"
LOOP
CLOSE #1
OPEN f FOR BINARY AS #1: PRINT "bytes"; LOF(1): CLOSE #1
PRINT "console WRITE"
WRITE 1, "a", 2.5
WRITE "with " + CHR$(34) + "quote", -1
WRITE
WRITE "ra", CHR$(-1), "rb"
PRINT "PRINT # not open"
PRINT #3, "x"
PRINT "WRITE # not open"
WRITE #3, "x"
PRINT "PRINT # to a file open for input"
OPEN f FOR INPUT AS #1
PRINT #1, "x"
CLOSE #1
PRINT "PRINT #1.5 (file 2)"
OPEN f FOR APPEND AS #2
PRINT #1.5, "float number"
CLOSE
PRINT "PRINT # with a raising number"
PRINT #ASC(""), "x"
PRINT "PRINT #0 and #-1"
PRINT #0, "x"
PRINT #-1, "x"
KILL f
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
