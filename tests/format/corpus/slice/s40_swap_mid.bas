$Console:Only
' SWAP and the MID$ statement (spec language/builtin-statements "SWAP", "MID$ statement"): SWAP of every numeric
' type, of two types that differ only in signedness, strings, fixed-length strings of equal and unequal length, a
' string with a fixed-length one, TYPE variables and elements, elements, members, a variable with an element or a
' member, the same place twice, an element with a bad index (error 9; the exchange is then made with the array's
' first element, as the old compiler's program does), in a SUB; the MID$ statement with every start and length,
' fixed-length, element and member targets, its own value, raising arguments.
On Error GoTo h
Type rec
    n As Long
    s As String * 3
    d As Double
End Type
Dim b1 As _Byte, b2 As _Byte, ub1 As _Unsigned _Byte, ub2 As _Unsigned _Byte
Dim i1 As Integer, i2 As Integer, ui1 As _Unsigned Integer, ui2 As _Unsigned Integer
Dim l1 As Long, l2 As Long, ul1 As _Unsigned Long, ul2 As _Unsigned Long
Dim q1 As _Integer64, q2 As _Integer64, uq1 As _Unsigned _Integer64, uq2 As _Unsigned _Integer64
Dim s1 As Single, s2 As Single, d1 As Double, d2 As Double, f1 As _Float, f2 As _Float
Dim o1 As _Offset, o2 As _Offset, uo1 As _Unsigned _Offset, uo2 As _Unsigned _Offset
Dim a As String, c As String, x1 As String * 3, x2 As String * 3, x5 As String * 5
Dim r1 As rec, r2 As rec, arr(3) As Long, sarr(3) As String, rarr(2) As rec, k As Long
Dim s As String, fx As String * 5, d As Double, n As Long

b1 = 1: b2 = 2: Swap b1, b2: Print "_BYTE"; b1; b2
ub1 = 1: ub2 = 200: Swap ub1, ub2: Print "_UNSIGNED _BYTE"; ub1; ub2
i1 = 1: i2 = 2: Swap i1, i2: Print "INTEGER"; i1; i2
ui1 = 1: ui2 = 60000: Swap ui1, ui2: Print "_UNSIGNED INTEGER"; ui1; ui2
l1 = 1: l2 = 2: Swap l1, l2: Print "LONG"; l1; l2
ul1 = 1: ul2 = 4000000000: Swap ul1, ul2: Print "_UNSIGNED LONG"; ul1; ul2
q1 = 1: q2 = 5000000000: Swap q1, q2: Print "_INTEGER64"; q1; q2
uq1 = 1: uq2 = 9000000000: Swap uq1, uq2: Print "_UNSIGNED _INTEGER64"; uq1; uq2
s1 = 1.5: s2 = 2.5: Swap s1, s2: Print "SINGLE"; s1; s2
d1 = 1.5: d2 = 2.5: Swap d1, d2: Print "DOUBLE"; d1; d2
f1 = 1.5: f2 = 2.5: Swap f1, f2: Print "_FLOAT"; f1; f2
o1 = 1: o2 = 2: Swap o1, o2: Print "_OFFSET"; o1; o2
uo1 = 1: uo2 = 2: Swap uo1, uo2: Print "_UNSIGNED _OFFSET"; uo1; uo2
l1 = -1: ul1 = 2: Swap l1, ul1: Print "LONG and _UNSIGNED LONG"; l1; ul1
b1 = -1: ub1 = 2: Swap ub1, b1: Print "_UNSIGNED _BYTE and _BYTE"; ub1; b1
a = "left": c = "r": Swap a, c: Print "STRING ["; a; "]["; c; "]"
x1 = "abc": x2 = "xy": Swap x1, x2: Print "STRING * 3 ["; x1; "]["; x2; "]"
x1 = "abc": x5 = "vwxyz": Swap x1, x5: Print "STRING * 3 and * 5 ["; x1; "]["; x5; "]"
a = "long one": x1 = "abc": Swap a, x1: Print "STRING and STRING * 3 ["; a; "]["; x1; "]"
r1.n = 1: r1.s = "one": r1.d = 1.5: r2.n = 2: r2.s = "two": r2.d = 2.5
Swap r1, r2: Print "TYPE"; r1.n; r1.s; r1.d; r2.n; r2.s; r2.d
arr(1) = 7: arr(2) = 9: Swap arr(1), arr(2): Print "elements"; arr(1); arr(2)
k = 1: Swap arr(k), arr(k + 1): Print "elements by variable"; arr(1); arr(2)
sarr(0) = "p": sarr(3) = "q": Swap sarr(0), sarr(3): Print "string elements "; sarr(0); sarr(3)
l1 = 5: Swap l1, arr(1): Print "variable and element"; l1; arr(1)
Swap r1.n, l1: Print "member and variable"; r1.n; l1
Swap r1.n, r2.n: Print "members"; r1.n; r2.n
x1 = "abc": Swap r1.s, x1: Print "fixed member and variable ["; r1.s; "]["; x1; "]"
rarr(0).n = 10: rarr(1).n = 11: Swap rarr(0), rarr(1): Print "TYPE elements"; rarr(0).n; rarr(1).n
Swap rarr(0), r1: Print "TYPE element and variable"; rarr(0).n; r1.n
l1 = 3: Swap l1, l1: Print "the same variable"; l1
a = "same": Swap a, a: Print "the same string "; a
Swap made1, made2: Print "variables made by SWAP"; made1; made2
arr(0) = 0: arr(1) = 1: l1 = 2: k = 9
Print "an element with a bad index": Swap l1, arr(k): Print l1; arr(0); arr(1)
Print "a bad index first": Swap arr(k), l1: Print l1; arr(0); arr(1)
insub

s = "abcdef": Mid$(s, 3) = "XY": Print "no length ["; s; "]"
s = "abcdef": Mid$(s, 3, 1) = "XY": Print "length 1 ["; s; "]"
s = "abcdef": Mid$(s, 3, 5) = "XY": Print "length beyond the value ["; s; "]"
s = "abcdef": Mid$(s, 5) = "VWXYZ": Print "value beyond the end ["; s; "]"
s = "abcdef": Mid$(s, 5, 9) = "VWXYZ": Print "length and value beyond the end ["; s; "]"
s = "abcdef": Mid$(s, 1) = "": Print "empty value ["; s; "]"
s = "abcdef": Mid$(s, 6) = "Z": Print "last position ["; s; "]"
s = "abcdef": Mid$(s, 3, 0) = "XY": Print "length 0 ["; s; "]"
s = "q": Mid$(s, 1, 1) = "ZZ": Print "one byte ["; s; "]"
s = "abcdef": Print "start 7 (one past the end)": Mid$(s, 7) = "Z": Print "["; s; "]"
s = "abcdef": Print "start 0": Mid$(s, 0) = "Z": Print "["; s; "]"
s = "abcdef": Print "start -1": Mid$(s, -1) = "Z": Print "["; s; "]"
s = "abcdef": Print "length -1": Mid$(s, 2, -1) = "XYZ": Print "["; s; "]"
s = "": Print "empty target, start 1": Mid$(s, 1) = "Z": Print "["; s; "]"
s = "abcdef": d = 2.5: Mid$(s, d, d) = "XYZ": Print "start and length 2.5 ["; s; "]"
s = "abcdef": d = 3.5: Mid$(s, d) = "XYZ": Print "start 3.5 ["; s; "]"
s = "abcdef": Mid$(s, 2, 3) = s: Print "its own value ["; s; "]"
s = "abcdef": Mid$(s, 1, 2) = Mid$(s, 5, 2): Print "its own MID$ ["; s; "]"
fx = "abcde": Mid$(fx, 4) = "XYZ": Print "fixed-length ["; fx; "]"
sarr(1) = "abcdef": n = 1: Mid$(sarr(n), 2, 2) = "XY": Print "element ["; sarr(1); "]"
r1.s = "abc": Mid$(r1.s, 2, 2) = "XY": Print "member ["; r1.s; "]"
s = "abcdef": Print "raising value": Mid$(s, 2) = Chr$(-1): Print "["; s; "]"
s = "abcdef": Print "raising start": Mid$(s, Asc("")) = "Z": Print "["; s; "]"
n = 9: sarr(0) = "abcdef": Print "element with a bad index": Mid$(sarr(n), 1) = "Z": Print "["; sarr(0); "]"
s = "abcdef": Mid$(s, 2, 2) = "X" + "Y" + "Z": Print "an expression ["; s; "]"
s = "abcdef": Mid$(s, 2) = "sp": Print "a blank before ( ["; s; "]"
System
h:
Print "  error"; Err
Resume Next

Sub insub
    Dim p As Long, q As Long, t As String
    Static st As Long
    p = 1: q = 2: st = 3
    Swap p, q: Swap q, st
    t = "local": Mid$(t, 1, 1) = "L"
    Print "in a SUB"; p; q; st; t
End Sub
