$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Console input"): answers that do not fit their targets
' (v22_e_redo.stdin, one answer line per INPUT): fewer and more fields than targets, a field that is no number, a
' number too large for its target, a negative one for an unsigned target, an unclosed quote, text after a closing
' quote, a blank inside a number, a target whose index is out of range. The program never asks again: a character
' that does not fit is dropped as it is typed, and the text so far is printed again.
ON ERROR GOTO h
DIM l AS LONG, m AS LONG, b AS _BYTE, s AS STRING, t AS STRING, i AS INTEGER, ub AS _UNSIGNED _BYTE
DIM arr(3) AS LONG, k AS LONG, q AS _INTEGER64, sg AS SINGLE, bt AS _BIT * 3
PRINT "1 two targets, the answer is 5"
l = -1: m = -1
INPUT "two"; l, m
PRINT "<"; l; "><"; m; ">"
PRINT "2 one target, the answer is 1,2"
INPUT "one"; l
PRINT "<"; l; ">"
PRINT "3 a number target, the answer is abc"
l = -1
INPUT "num"; l
PRINT "<"; l; ">"
PRINT "4 a _BYTE target, the answer is 300"
INPUT "byte"; b
PRINT "<"; b; ">"
PRINT "5 an INTEGER target, the answer is 70000"
INPUT "int"; i
PRINT "<"; i; ">"
PRINT "6 an _UNSIGNED _BYTE target, the answer is -1"
INPUT "ubyte"; ub
PRINT "<"; ub; ">"
PRINT "7 a number target, the answer is 12abc"
INPUT "num"; l
PRINT "<"; l; ">"
PRINT "8 a number and a string, the answer is x,y"
INPUT "mixed"; l, s
PRINT "<"; l; "><"; s; ">"
PRINT "9 an unclosed quote into two strings, the answer is "; CHR$(34); "open, still open"
s = "old": t = "old"
INPUT "quote"; s, t
PRINT "<"; s; "><"; t; ">"
PRINT "10 text after a closing quote, the answer is "; CHR$(34); "a"; CHR$(34); " b"
INPUT "after"; s
PRINT "<"; s; ">"
PRINT "11 a number target, the answer is 1 2"
INPUT "blank"; l
PRINT "<"; l; ">"
PRINT "12 a LONG target, the answer is 99999999999"
INPUT "long"; l
PRINT "<"; l; ">"
PRINT "13 an _INTEGER64 target, the answer is 99999999999999999999"
INPUT "int64"; q
PRINT "<"; q; ">"
PRINT "14 a SINGLE target, the answer is 1e50"
INPUT "single"; sg
PRINT "<"; sg; ">"
PRINT "15 an element with a bad index, the answer is 11"
k = 9: arr(0) = -7
INPUT "elem"; arr(k)
PRINT "<"; arr(0); ">"
PRINT "16 the next INPUT, the answer is 12"
INPUT "next"; l
PRINT "<"; l; ">"
PRINT "17 a _BIT * 3 variable holding 2, the answer is 1"
bt = 2
INPUT "bit"; bt
PRINT "<"; bt; ">"
PRINT "18 one target and a comma after it, the answer is 13"
INPUT "comma"; l,
PRINT "<"; l; ">"
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
