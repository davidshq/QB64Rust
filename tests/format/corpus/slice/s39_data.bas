$Console:Only
' DATA, READ and RESTORE (spec language/data-read): the program's data in file order (main module, a block that
' never runs, after SYSTEM, in a SUB and a FUNCTION), bare DATA, quoted and unquoted items; READ into every numeric
' type, strings, fixed-length strings, elements and members; number forms; items that are no number (error 2) or
' too large (error 6) are not taken, and the later targets of that READ get 0; out of data (error 4); RESTORE to
' the start and to labels of any body.
On Error GoTo h
Type rec
    n As Long
    d As Double
    f As String * 4
End Type
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer, l As Long, ul As _Unsigned Long
Dim q As _Integer64, uq As _Unsigned _Integer64, sg As Single, db As Double, fl As _Float
Dim o As _Offset, uo As _Unsigned _Offset, bt As _Bit, b3 As _Bit * 3, ub3 As _Unsigned _Bit * 3
Dim s As String, t As String, fx As String * 4, a(3) As Long, sa(3) As String, r As rec, k As Long
Dim Shared skipit As Long

first:
Data m1,"m 2"
If 0 Then
    Data in_if
End If
Print "the order": Data after_colon
Data
Data "q",,last
For k = 1 To 9
    Read s
    Print k; "["; s; "]"
Next

Print "every type from 1": Restore ones
Read b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
Print b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3
Print "2.5 into each": Restore halves
Read b, ub, i, ui, l, ul, q, uq, sg, db, fl, o, uo, b3, ub3
Print b; ub; i; ui; l; ul; q; uq; sg; db; fl; o; uo; b3; ub3
Print "-1 into a _BIT and into the signed types": Restore minus
Read bt, b, i, l, q, sg, db, fl, o, b3
Print bt; b; i; l; q; sg; db; fl; o; b3

skipit = -1
Print "number forms into DOUBLE": Restore forms
For k = 1 To 14: db = -7: Read db: Print db;: Next: Print
Print "number forms into LONG": Restore forms
For k = 1 To 14: l = -7: Read l: Print l;: Next: Print
Print "number forms into _INTEGER64": Restore forms
For k = 1 To 14: q = -7: Read q: Print q;: Next: Print
Print "number forms into _UNSIGNED _INTEGER64": Restore forms
For k = 1 To 14: uq = 7: Read uq: Print uq;: Next: Print
Print "number forms into STRING": Restore forms
For k = 1 To 14: Read s: Print "["; s; "]";: Next: Print
Print "no number": Restore nonum
For k = 1 To 8: l = -7: Read l: Print k; l: Next
Print "too large": Restore large
b = -7: Read b: Print "_BYTE 300:"; b
ub = 7: Read ub: Print "_UNSIGNED _BYTE -1:"; ub
i = -7: Read i: Print "INTEGER 70000:"; i
l = -7: Read l: Print "LONG 5000000000:"; l
q = -7: Read q: Print "_INTEGER64 9223372036854775808:"; q
uq = 7: Read uq: Print "_UNSIGNED _INTEGER64 18446744073709551615:"; uq
uq = 7: Read uq: Print "_UNSIGNED _INTEGER64 18446744073709551616:"; uq
sg = -7: Read sg: Print "SINGLE 1e39:"; sg
db = -7: Read db: Print "DOUBLE 1e309:"; db
bt = 0: Read bt: Print "_BIT 2:"; bt
ub3 = 0: Read ub3: Print "_UNSIGNED _BIT * 3 9:"; ub3
Read q: Print "_INTEGER64 9007199254740993:"; q
skipit = 0

Print "strings": Restore strs
Read s: Print "["; s; "]"
Read s: Print "["; s; "]"
Read s, t: Print "["; s; "]["; t; "]"
Read fx: Print "["; fx; "]"
Read fx: Print "["; fx; "]"
Print "element, member": Restore places
k = 2
Read a(k), sa(k + 1), r.n, r.d, r.f
Print a(2); "["; sa(3); "]"; r.n; r.d; "["; r.f; "]"

Print "an item too large in the middle: it stays": Restore toolarge
l = -7: b = -7: i = -7
Read l, b, i
Print l; b; i
Read i: Print "next item:"; i
Print "no number in the middle: it stays": Restore notnum
l = -7: i = -7: q = -7: s = "old"
Read l, i, q, s
Print l; i; q; "["; s; "]"
Read s: Print "next item: "; s
Print "an element out of range in the middle": Restore places
l = -7: i = -7: k = 9
Read l, a(k), i
Print l; i
Read s: Print "next item: "; s

Print "out of data": Restore tail
Read l: Print l
l = -7: i = -7: s = "old": t = "old"
Read l, i, s
Print l; i; "["; s; "]"
Read s: Print "["; s; "]"
Read i, t: Print i; "["; t; "]"
Print "RESTORE, then the first item": Restore: Read s: Print s
Print "RESTORE to a label on a DATA line": Restore strs2: Read s: Print s
Print "RESTORE to a label of a SUB": Restore insub: Read s: Print s
Print "RESTORE to a label with no DATA after it": Restore nothing: s = "old": Read s: Print "["; s; "]"
Print "RESTORE FIRST": Restore first: Read s: Print s
p
Read s: Print "back in main: "; s
Print f&
System
Data after_system
h:
Print "  error"; Err
If skipit <> 0 And Err <> 4 Then Read junk$
Resume Next

ones:
Data 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1
halves:
Data 2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5,2.5
minus:
Data -1,-1,-1,-1,-1,-1,-1,-1,-1,-1
forms:
Data &H10,&O17,&B101,1e3,1d3,1.5E+2,"12"," 12 ",+5,.5,-.5,1f2,&HFFFF,&HFFFFFFFF
nonum:
Data x,"abc",12abc,,"",1 2,1e,--1
large:
Data 300,-1,70000,5000000000,9223372036854775808
Data 18446744073709551615,18446744073709551616,1e39,1e309,2,9,9007199254740993
strs:
Data bare  word,"  quoted, with comma  ",,""
strs2: Data ab,abcdefgh,"unclosed, to the end"
places:
Data 5,elem,6,7.5,member
toolarge:
Data 10,300,30,40
notnum:
Data 11,zz,33,44

Sub p
    Dim s As String
    Print "in p: RESTORE to a main-module label": Restore strs: Read s: Print s
    insub:
    Data in_sub
    Print "in p: RESTORE to its own label": Restore insub: Read s: Print s
    Print "in p: RESTORE": Restore: Read s: Print s
End Sub

Function f&
    Data 77
    tail:
    Data 51,52
    nothing:
    Restore insub
    Read s$, l&: f& = l&
End Function
