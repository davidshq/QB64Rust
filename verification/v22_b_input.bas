$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Files"): INPUT # into every numeric type and strings: quoted and unquoted
' fields, blanks, an empty field, a field that is no number, a number too large for its target, fewer fields than
' targets, CR LF against LF; LINE INPUT # of an empty line, of a last line without a line end, past the end; targets
' that are elements, members and fixed-length strings. Each case writes the file anew (SUB setfile) and reads it.
ON ERROR GOTO h
DIM SHARED f AS STRING
DIM a AS STRING, c AS STRING, lf AS STRING, crlf AS STRING, qt AS STRING
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
f = "v22_b_i.tmp": lf = CHR$(10): crlf = CHR$(13) + CHR$(10): qt = CHR$(34)
PRINT "every integer type"
setfile "1,2,3,4,5,6,7,8"
INPUT #1, b, ub, i, ui, l, ul, q, uq
PRINT b; ub; i; ui; l; ul; q; uq
PRINT "floats, _OFFSET, _BIT"
setfile "1.5,2.5,3.5,12,-3"
INPUT #1, s, d, fl, o, bt
PRINT s; d; fl; o; bt
PRINT "a WRITE line"
setfile "1," + qt + "a,b" + qt + ",2.5"
INPUT #1, l, a, d
PRINT l; a; d
PRINT "200 into a _BYTE, then the next target"
setfile "200,7": b = 1: l = 1
INPUT #1, b, l
PRINT b; l
INPUT #1, a: PRINT "the next field is ["; a; "]"
PRINT "one value per case: 300 %%, 70000 %, -1 ~%%, 5000000000 &, -1 ~&, 99999999999999999999 &&, 2.5 %, 3.5 %"
setfile "300": b = 1: INPUT #1, b: PRINT b
setfile "70000": i = 1: INPUT #1, i: PRINT i
setfile "-1": ub = 1: INPUT #1, ub: PRINT ub
setfile "5000000000": l = 1: INPUT #1, l: PRINT l
setfile "-1": ul = 1: INPUT #1, ul: PRINT ul
setfile "99999999999999999999": q = 1: INPUT #1, q: PRINT q
setfile "-1": uq = 1: INPUT #1, uq: PRINT uq
setfile "18446744073709551615": uq = 1: INPUT #1, uq: PRINT uq
setfile "2.5": i = 1: INPUT #1, i: PRINT i
setfile "3.5": i = 1: INPUT #1, i: PRINT i
setfile "2.5": q = 1: INPUT #1, q: PRINT q
setfile "20": bt = 1: INPUT #1, bt: PRINT bt
setfile "1e40": s = 1: INPUT #1, s: PRINT s
setfile "1d400": d = 1: INPUT #1, d: PRINT d
setfile "1e40": d = 1: INPUT #1, d: PRINT d
PRINT "text for a number: abc, 12x, &H10, &HFFFF into %, empty, blanks, 1 2, 1e2, 1d2, .5, -, +5"
setfile "abc": l = 1: INPUT #1, l: PRINT l
setfile "12x": l = 1: INPUT #1, l: PRINT l
setfile "&H10": l = 1: INPUT #1, l: PRINT l
setfile "&HFFFF": i = 1: INPUT #1, i: PRINT i
setfile "": l = 1: INPUT #1, l: PRINT l
setfile ",5": l = 1: INPUT #1, l: PRINT l
setfile "   ,5": l = 1: INPUT #1, l: PRINT l
setfile "1 2": l = 1: INPUT #1, l: PRINT l;: INPUT #1, l: PRINT l
setfile "1e2": l = 1: INPUT #1, l: PRINT l
setfile "1d2": l = 1: INPUT #1, l: PRINT l
setfile ".5": s = 1: INPUT #1, s: PRINT s
setfile "-": l = 1: INPUT #1, l: PRINT l
setfile "+5": l = 1: INPUT #1, l: PRINT l
setfile qt + "12" + qt: l = 1: INPUT #1, l: PRINT l
PRINT "strings: blanks around, quotes, a comma inside quotes, text after the closing quote, an empty field"
setfile "  lead , trail  ," + qt + " quoted, with comma " + qt + ", after"
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]"
setfile qt + "q" + qt + "tail,next"
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]"
setfile "a,,b"
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]"
setfile qt + "open quote to the end"
INPUT #1, a: PRINT "["; a; "]"
PRINT "fewer fields than targets: the next lines are taken"
setfile "7" + crlf + "8" + crlf + "9,10"
INPUT #1, l, i, s, d
PRINT l; i; s; d
PRINT "LF, CR and CR LF as line ends"
setfile "lf1" + lf + "lf2" + lf + "cr1" + CHR$(13) + "cr2" + crlf + "end"
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]";
INPUT #1, a: PRINT "["; a; "]"
PRINT "element, member and fixed-length targets"
setfile "s1,s2,21,22,23.5,xyzzy"
INPUT #1, sa(1), fx, arr(2), p.x, p.y, p.t
PRINT "["; sa(1); "]["; fx; "]"; arr(2); p.x; p.y; "["; p.t; "]"
PRINT "a raising target index: the later targets are not read"
setfile "31,32,33": l = 0: i = 0
INPUT #1, l, arr(9), i
PRINT l; i
INPUT #1, c: PRINT "the next field is ["; c; "]"
setfile "s1,s2,s3": a = "": c = ""
INPUT #1, a, sa(9), c
PRINT "["; a; "]["; c; "]["; sa(0); "]"
INPUT #1, c: PRINT "the next field is ["; c; "]"
PRINT "LINE INPUT: a line, an empty line, a line with commas and quotes, the last line without an end, past the end"
setfile "first" + crlf + crlf + "a, " + qt + "b" + qt + " ,c  " + crlf + "last line without end"
LINE INPUT #1, a: PRINT "["; a; "]"
LINE INPUT #1, a: PRINT "["; a; "]"
LINE INPUT #1, a: PRINT "["; a; "]"; EOF(1)
LINE INPUT #1, fx: PRINT "["; fx; "]"; EOF(1)
a = "kept": LINE INPUT #1, a: PRINT "["; a; "]"
a = "kept": INPUT #1, a: PRINT "["; a; "]"
l = 5: INPUT #1, l: PRINT l
PRINT "LINE INPUT with LF line ends, into an element and a member"
setfile "one" + lf + "two" + lf + "three"
LINE INPUT #1, sa(2): PRINT "["; sa(2); "]"
LINE INPUT #1, p.t: PRINT "["; p.t; "]"
CLOSE
PRINT "not open"
INPUT #3, a
LINE INPUT #3, a
PRINT "a file open for output"
OPEN f FOR OUTPUT AS #1
INPUT #1, a
LINE INPUT #1, a
CLOSE #1
PRINT "an empty file"
setfile ""
INPUT #1, a
LINE INPUT #1, a
PRINT "INPUT #1.5 (file 2) and a raising number"
INPUT #1.5, a
INPUT #ASC(""), a
PRINT "a binary file"
CLOSE: OPEN f FOR OUTPUT AS #1: PRINT #1, "bin,ary": CLOSE: OPEN f FOR BINARY AS #1
INPUT #1, a: PRINT "["; a; "]"
LINE INPUT #1, a: PRINT "["; a; "]"
CLOSE
KILL f
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT

SUB setfile (text AS STRING)
    CLOSE
    OPEN f FOR OUTPUT AS #1
    PRINT #1, text;
    CLOSE #1
    OPEN f FOR INPUT AS #1
END SUB
