$CONSOLE:ONLY
' Console INPUT and LINE INPUT, and RANDOMIZE without a seed (spec language/builtin-statements "Console INPUT and
' LINE INPUT", "RANDOMIZE"), with the answers in s42_console_input.stdin, one line per statement that reads: every
' prompt form, the ; before the prompt, each numeric type, strings with blanks and quotes, an empty line, a
' fixed-length string, elements and members; answers that do not fit (the program never asks again: a character
' that does not fit is dropped and the text so far printed again); a _BIT variable, which keeps its value; an
' element with a bad index, which raises 9 and reads nothing. The program ends with SYSTEM: END would wait for a key.
ON ERROR GOTO h
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT, o AS _OFFSET
DIM s AS STRING, t AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING, da(3) AS DOUBLE, m AS LONG
DIM bt AS _BIT * 3, k AS LONG
TYPE rec
    n AS LONG
    f AS STRING * 4
    d AS DOUBLE
END TYPE
DIM r AS rec
PRINT "1 no prompt"
INPUT l
PRINT "<"; l; ">"
PRINT "2 a prompt and ;"
INPUT "number"; l
PRINT "<"; l; ">"
PRINT "3 a prompt and ,"
INPUT "number: ", l
PRINT "<"; l; ">"
PRINT "4 an empty prompt and ,"
INPUT "", l
PRINT "<"; l; ">"
PRINT "5 ; before the prompt: the output stays on the line"
INPUT ; "stay"; l
PRINT "<"; l; ">"
INPUT ; l
PRINT "<"; l; ">"
INPUT ; "stay: ", l
PRINT "<"; l; ">"
PRINT "6 three targets"
INPUT "three"; l, s, db
PRINT "<"; l; "><"; s; "><"; db; ">"
PRINT "7 a quoted field with a comma, blanks around fields"
INPUT s, t
PRINT "<"; s; "><"; t; ">"
INPUT l, s
PRINT "<"; l; "><"; s; ">"
PRINT "8 an empty line into a number and into a string"
l = 5: s = "old"
INPUT l
PRINT "<"; l; ">"
INPUT s
PRINT "<"; s; ">"
PRINT "9 every numeric type"
INPUT b, ub, i, ui, l, ul
PRINT b; ub; i; ui; l; ul
INPUT q, uq, sg, db, fl, o
PRINT q; uq; sg; db; fl; o
PRINT "10 large values"
INPUT l, ul, q, uq
PRINT l; ul; q; uq
INPUT sg, db
PRINT sg; db
PRINT "11 number forms: &H10, an exponent, D, no digit before the point, signs"
INPUT l, sg, db, sg, db, i
PRINT l; sg; db; i
PRINT "12 a fixed-length string, elements, members"
k = 2
INPUT fx, a(k), sa(k), da(k + 1), r.n, r.f, r.d
PRINT "<"; fx; "><"; a(2); "><"; sa(2); "><"; da(3); "><"; r.n; "><"; r.f; "><"; r.d; ">"
PRINT "13 LINE INPUT: no prompt, a prompt with ; and with , and ; before it"
LINE INPUT s
PRINT "<"; s; ">"
LINE INPUT "line; "; s
PRINT "<"; s; ">"
LINE INPUT "line, ", s
PRINT "<"; s; ">"
LINE INPUT ; "stay "; s
PRINT "<"; s; ">"
PRINT "14 LINE INPUT of an empty line, into a fixed-length string, an element, a member"
s = "old"
LINE INPUT s
PRINT "<"; s; ">"
LINE INPUT fx
LINE INPUT sa(k)
LINE INPUT r.f
PRINT "<"; fx; "><"; sa(2); "><"; r.f; ">"
PRINT "15 an implicit variable, a comma after the last target"
INPUT made
PRINT "<"; made; ">"
INPUT l,
PRINT "<"; l; ">"
LINE INPUT s,
PRINT "<"; s; ">"
PRINT "16 fewer fields than targets, more fields than targets"
l = -1: m = -1
INPUT "two"; l, m
PRINT "<"; l; "><"; m; ">"
INPUT "one"; l
PRINT "<"; l; ">"
PRINT "17 no number, a float for an integer, a blank inside a number"
l = -1
INPUT "num"; l
PRINT "<"; l; ">"
INPUT "num"; l
PRINT "<"; l; ">"
INPUT "num"; b, i, l, q
PRINT b; i; l; q
INPUT "blank"; l
PRINT "<"; l; ">"
PRINT "18 too large for the target, a minus for an unsigned target"
INPUT "byte"; b
PRINT "<"; b; ">"
INPUT "int"; i
PRINT "<"; i; ">"
INPUT "long"; l
PRINT "<"; l; ">"
INPUT "int64"; q
PRINT "<"; q; ">"
INPUT "single"; sg
PRINT "<"; sg; ">"
INPUT "ubyte"; ub
PRINT "<"; ub; ">"
PRINT "19 a number and a string given a letter, an unclosed quote, text after a closing quote"
INPUT "mixed"; l, s
PRINT "<"; l; "><"; s; ">"
s = "old": t = "old"
INPUT "quote"; s, t
PRINT "<"; s; "><"; t; ">"
INPUT "after"; s
PRINT "<"; s; ">"
PRINT "20 a _BIT variable keeps its value"
bt = 2
INPUT "bit"; bt
PRINT "<"; bt; ">"
PRINT "21 an element with a bad index: error 9, nothing is read"
k = 9: a(0) = -7
INPUT "elem"; a(k)
PRINT "<"; a(0); ">"
INPUT "next"; l
PRINT "<"; l; ">"
PRINT "22 RANDOMIZE without a seed"
RANDOMIZE
PRINT RND; RND
RANDOMIZE
PRINT RND
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
