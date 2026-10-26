$Console:Only
' Sequential files (spec language/file-io): OPEN in both forms, PRINT # with every item kind and separator, WRITE
' to a file and to the console, APPEND, then the file read back with LINE INPUT # and INPUT # into every kind of
' target. Zones in a file are 14 columns; a number is its STR$ and a blank; WRITE puts strings in quotes and numbers
' without blanks; INPUT # reads one field per target. The file is s36_*.tmp and removed again.
Dim f As String, a As String, c As String, k As Long, n As Long
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer, l As Long, ul As _Unsigned Long
Dim q As _Integer64, uq As _Unsigned _Integer64, s As Single, d As Double, fl As _Float, o As _Offset
Dim bt As _Bit * 5, fx As String * 4
Dim arr(3) As Long, sa(3) As String
Type pt
    x As Long
    y As Double
    t As String * 3
End Type
Dim p As pt
b = -5: ub = 250: i = -300: ui = 65000: l = -70000: ul = 4000000000: q = -5000000000: uq = 18446744073709551615~&&
s = 1.5: d = -2.25: fl = 1E+20: o = 12: bt = -3: fx = "ab"
f = "s36_files.tmp"

Open f For Output As #1
Print #1, "x"; 7; "y"
Print #1, 1; -2; 3.5
Print #1, "a", "b", 5
Print #1, "12345678901234", "z"
Print #1, "1234567890123", "z"
Print #1, , "after a zone"
Print #1, "no newline";
Print #1, " same line"
Print #1, "zone at the end",
Print #1, "next"
Print #1,
Print #1, ;
Print #1, b; ub; i; ui; l; ul; q; uq
Print #1, s; d; fl; o; bt; "["; fx; "]"
Print #1, Left$("left", 2) + "!"; Len(a) + 1
Write #1, 1, "a", 2.5
Write #1, b, ub, i, ui, l, ul, q, uq
Write #1, s, d, fl, o, bt, fx
Write #1, -1, -2.5, 1E+30, .5, -.5, 100000000
Write #1, "with " + Chr$(34) + "quote", "comma, inside", ""
Write #1,
Write #1, "open",
Write #1, "closed"
Close #1

n = 2
Open f For Append As n
Print #n, "appended through a variable"
Print #1.5, "and through 1.5, which is file 2"
Close n

Open "A", #3, f
Print #3, "old form A"
Close

Open f For Input As #1
Do Until EOF(1)
    Line Input #1, a
    k = k + 1
    Print k; "["; a; "]"
Loop
Close #1

Print "console WRITE"
Write 1, "a", 2.5
Write b, s, fx
Write
Write "open",
Write "closed"

Print "INPUT # into every type"
Open f For Output As #1
Write #1, 1, 2, 3, 4, 5, 6, 7, 8
Write #1, 1.5, 2.5, 3.5, 12, -3
Write #1, "plain", "with, comma", 42
Print #1, "  lead , trail  ,7"
Print #1, "2.5,3.5,&H10,12x,,"
Print #1, "9"
Print #1, "10"
Print #1, "s1,s2,21,22,23.5,xyzzy"
Print #1, "a line, with a comma"
Print #1, "fixed length"
Print #1, "last line without an end";
Close #1
Open "I", 1, f
Input #1, b, ub, i, ui, l, ul, q, uq
Print b; ub; i; ui; l; ul; q; uq
Input #1, s, d, fl, o, bt
Print s; d; fl; o; bt
Input #1, a, c, l
Print "["; a; "]["; c; "]"; l
Input #1, a, c, l
Print "["; a; "]["; c; "]"; l
Input #1, i, l, ui, n, s, a
Print i; l; ui; n; s; "["; a; "]"
Input #1, i, l
Print i; l
Input #1, sa(1), fx, arr(2), p.x, p.y, p.t
Print "["; sa(1); "]["; fx; "]"; arr(2); p.x; p.y; "["; p.t; "]"
Line Input #1, sa(2)
Print "["; sa(2); "]"
Line Input #1, fx
Print "["; fx; "]"; EOF(1)
Line Input #1, a
Print "["; a; "]"; EOF(1)
Close
Kill f
System
