$Console:Only
' Console INPUT and LINE INPUT, and RANDOMIZE without a seed (spec language/builtin-statements "Console INPUT and
' LINE INPUT", "RANDOMIZE"), with the answers in s42_console_input.stdin, one line per statement that reads: every
' prompt form, the ; before the prompt, each numeric type, strings with blanks and quotes, an empty line, a
' fixed-length string, elements and members; answers that do not fit (the program never asks again: a character
' that does not fit is dropped and the text so far printed again); a _BIT variable, which keeps its value; an
' element with a bad index, which raises 9 and reads nothing. The program ends with SYSTEM: END would wait for a key.
On Error GoTo h
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer, l As Long, ul As _Unsigned Long
Dim q As _Integer64, uq As _Unsigned _Integer64, sg As Single, db As Double, fl As _Float, o As _Offset
Dim s As String, t As String, fx As String * 4, a(3) As Long, sa(3) As String, da(3) As Double, m As Long
Dim bt As _Bit * 3, k As Long
Type rec
    n As Long
    f As String * 4
    d As Double
End Type
Dim r As rec
Print "1 no prompt"
Input l
Print "<"; l; ">"
Print "2 a prompt and ;"
Input "number"; l
Print "<"; l; ">"
Print "3 a prompt and ,"
Input "number: ", l
Print "<"; l; ">"
Print "4 an empty prompt and ,"
Input "", l
Print "<"; l; ">"
Print "5 ; before the prompt: the output stays on the line"
Input ; "stay"; l
Print "<"; l; ">"
Input ; l
Print "<"; l; ">"
Input ; "stay: ", l
Print "<"; l; ">"
Print "6 three targets"
Input "three"; l, s, db
Print "<"; l; "><"; s; "><"; db; ">"
Print "7 a quoted field with a comma, blanks around fields"
Input s, t
Print "<"; s; "><"; t; ">"
Input l, s
Print "<"; l; "><"; s; ">"
Print "8 an empty line into a number and into a string"
l = 5: s = "old"
Input l
Print "<"; l; ">"
Input s
Print "<"; s; ">"
Print "9 every numeric type"
Input b, ub, i, ui, l, ul
Print b; ub; i; ui; l; ul
Input q, uq, sg, db, fl, o
Print q; uq; sg; db; fl; o
Print "10 large values"
Input l, ul, q, uq
Print l; ul; q; uq
Input sg, db
Print sg; db
Print "11 number forms: &H10, an exponent, D, no digit before the point, signs"
Input l, sg, db, sg, db, i
Print l; sg; db; i
Print "12 a fixed-length string, elements, members"
k = 2
Input fx, a(k), sa(k), da(k + 1), r.n, r.f, r.d
Print "<"; fx; "><"; a(2); "><"; sa(2); "><"; da(3); "><"; r.n; "><"; r.f; "><"; r.d; ">"
Print "13 LINE INPUT: no prompt, a prompt with ; and with , and ; before it"
Line Input s
Print "<"; s; ">"
Line Input "line; "; s
Print "<"; s; ">"
Line Input "line, ", s
Print "<"; s; ">"
Line Input ; "stay "; s
Print "<"; s; ">"
Print "14 LINE INPUT of an empty line, into a fixed-length string, an element, a member"
s = "old"
Line Input s
Print "<"; s; ">"
Line Input fx
Line Input sa(k)
Line Input r.f
Print "<"; fx; "><"; sa(2); "><"; r.f; ">"
Print "15 an implicit variable, a comma after the last target"
Input made
Print "<"; made; ">"
Input l,
Print "<"; l; ">"
Line Input s,
Print "<"; s; ">"
Print "16 fewer fields than targets, more fields than targets"
l = -1: m = -1
Input "two"; l, m
Print "<"; l; "><"; m; ">"
Input "one"; l
Print "<"; l; ">"
Print "17 no number, a float for an integer, a blank inside a number"
l = -1
Input "num"; l
Print "<"; l; ">"
Input "num"; l
Print "<"; l; ">"
Input "num"; b, i, l, q
Print b; i; l; q
Input "blank"; l
Print "<"; l; ">"
Print "18 too large for the target, a minus for an unsigned target"
Input "byte"; b
Print "<"; b; ">"
Input "int"; i
Print "<"; i; ">"
Input "long"; l
Print "<"; l; ">"
Input "int64"; q
Print "<"; q; ">"
Input "single"; sg
Print "<"; sg; ">"
Input "ubyte"; ub
Print "<"; ub; ">"
Print "19 a number and a string given a letter, an unclosed quote, text after a closing quote"
Input "mixed"; l, s
Print "<"; l; "><"; s; ">"
s = "old": t = "old"
Input "quote"; s, t
Print "<"; s; "><"; t; ">"
Input "after"; s
Print "<"; s; ">"
Print "20 a _BIT variable keeps its value"
bt = 2
Input "bit"; bt
Print "<"; bt; ">"
Print "21 an element with a bad index: error 9, nothing is read"
k = 9: a(0) = -7
Input "elem"; a(k)
Print "<"; a(0); ">"
Input "next"; l
Print "<"; l; ">"
Print "22 RANDOMIZE without a seed"
Randomize
Print Rnd; Rnd
Randomize
Print Rnd
Print "done"
System
h:
Print "  error"; Err
Resume Next
