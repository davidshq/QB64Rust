$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Console input"): INPUT and LINE INPUT from a redirected standard input
' (v22_e_input.stdin): prompts ended by ; and by , and no prompt, the leading ;, several targets, each numeric
' type, strings with blanks and quotes, an empty line, LINE INPUT of an empty line and of a line with commas.
' Each answer line of the .stdin file is named in the PRINT before it.
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT
DIM s AS STRING, t AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING
TYPE rec
    n AS LONG
    f AS STRING * 4
END TYPE
DIM r AS rec, k AS LONG
PRINT "1 no prompt, answer 42"
INPUT l
PRINT "<"; l; ">"
PRINT "2 prompt with ; answer 43"
INPUT "number"; l
PRINT "<"; l; ">"
PRINT "3 prompt with , answer 44"
INPUT "number: ", l
PRINT "<"; l; ">"
PRINT "4 leading ; answer 45, then text on the same line"
INPUT ; "stay"; l
PRINT "<"; l; ">"
PRINT "5 leading ; without a prompt, answer 46"
INPUT ; l
PRINT "<"; l; ">"
PRINT "6 three targets, answer 1, two words ,3.5"
INPUT "three"; l, s, db
PRINT "<"; l; "><"; s; "><"; db; ">"
PRINT "7 a quoted field with a comma, answer "; CHR$(34); "a, b"; CHR$(34); ",c"
INPUT s, t
PRINT "<"; s; "><"; t; ">"
PRINT "8 blanks around fields, answer '  7  ,  x y  '"
INPUT l, s
PRINT "<"; l; "><"; s; ">"
PRINT "9 an empty line into a number and a string"
INPUT l
PRINT "<"; l; ">"
INPUT s
PRINT "<"; s; ">"
PRINT "10 every numeric type, answer 1,2,3,4,5,6,7,8,1.5,2.5,3.5"
INPUT b, ub, i, ui, l, ul, q, uq, sg, db, fl
PRINT b; ub; i; ui; l; ul; q; uq; sg; db; fl
PRINT "11 2.5 into each integer type"
INPUT b, i, l, q
PRINT b; i; l; q
PRINT "12 number forms, answer &H10,1e3,1d3,.5,-.5,+5"
INPUT l, sg, db, sg, db, i
PRINT l; sg; db; i
PRINT "13 a fixed-length string, an element, a string element, members; answer abcdefgh,5,el,6,memb"
k = 2
INPUT fx, a(k), sa(k), r.n, r.f
PRINT "<"; fx; "><"; a(2); "><"; sa(2); "><"; r.n; "><"; r.f; ">"
PRINT "14 LINE INPUT without a prompt, answer 'a, b  c'"
LINE INPUT s
PRINT "<"; s; ">"
PRINT "15 LINE INPUT with a prompt and ; answer '  spaced  '"
LINE INPUT "line; "; s
PRINT "<"; s; ">"
PRINT "16 LINE INPUT with a prompt and , answer q"
LINE INPUT "line, ", s
PRINT "<"; s; ">"
PRINT "17 LINE INPUT with a leading ; answer r, then text on the same line"
LINE INPUT ; "stay "; s
PRINT "<"; s; ">"
PRINT "18 LINE INPUT of an empty line"
s = "old"
LINE INPUT s
PRINT "<"; s; ">"
PRINT "19 LINE INPUT into a fixed-length string and an element, answers abcdefgh and elem"
LINE INPUT fx
LINE INPUT sa(k)
PRINT "<"; fx; "><"; sa(2); ">"
PRINT "20 an implicit variable, answer 9"
INPUT made
PRINT "<"; made; ">"
PRINT "done"
SYSTEM
