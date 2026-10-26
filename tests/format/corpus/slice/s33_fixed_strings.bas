$Console:Only
' Slice program (m2-numeric-types, tasks 7.1 and 7.2): fixed-length strings, the scenarios of the
' fixed-length-strings spec delta and the forms measured by verification\v21_c_*: declarations (AS STRING * n with a
' number or a constant, _UNSIGNED STRING * n, the suffix form name$n in DIM and implicit, a length read in 32 bits),
' NUL bytes at the start (also a local on each call, DIVERGENCES-QB45.md Q-005), stores that pad and cut, comparisons
' and SELECT CASE with the padding, built-ins, members, array elements, members of elements, each storage class, and
' fixed strings passed to a STRING parameter by reference (also in parentheses). NUL bytes are shown with ASC, never
' printed. The MID$ statement and FUNCTIONs named name$n are not supported yet. No PRINT comma.
On Error GoTo h
' Declarations and the initial bytes.
Dim f As String * 4
Print "initial:"; Len(f); Asc(f, 1); Asc(f, 4)
Dim w As String * 4294967297
Print "4294967297:"; Len(w)
Const n = 3
Dim c As String * n
c = "abcdef"
Print "STRING * CONST: ["; c; "]"; Len(c)
Dim u As _Unsigned String * 3
u = "abcdef"
Print "_UNSIGNED STRING * 3: ["; u; "]"; Len(u)
Dim p$3
p$3 = "abcdef": p$ = "x"
Print "suffix: "; p$3; Len(p$3); p$
q$2 = "xyz"
Print "implicit q$2: ["; q$2; "]"; Len(q$2)
' Stores: padded with blanks, cut to n.
f = "ab"
Print "short: ["; f; "]"; Asc(f, 3); Len(f)
f = "abcdef"
Print "long: ["; f; "]"
f = ""
Print "empty: ["; f; "]"; Asc(f, 1)
f = "ab"
f = f + "x"
Print "f = f + x: ["; f; "]"
f = "abcd"
f = Mid$(f, 2)
Print "f = MID$(f, 2): ["; f; "]"
Dim g As String * 6
g = "xy": f = g
Print "fixed from longer fixed: ["; f; "]"
g = f
Print "fixed from shorter fixed: ["; g; "]"
' Values: comparisons see the padding.
f = "ab"
Print "compare:"; f = "ab"; f = "ab  "; f < "ac"; f > "ab"; "ab  " = f
Dim s As String
s = f
Print "into STRING: ["; s; "]"; Len(s)
Print "concat: ["; f + "!"; "]"
Select Case f
    Case "ab": Print "case: ab"
    Case "ab  ": Print "case: ab and 2 blanks"
    Case Else: Print "case: else"
End Select
' Built-ins.
Dim k As String * 5
k = "ab"
Print "built-ins:"; Len(RTrim$(k)); " ["; UCase$(k); "] ["; Left$(k, 3); "] ["; Right$(k, 2); "] ["; Mid$(k, 2, 3); "]"
Print "more:"; InStr(k, " "); InStr(k, "b"); Asc(k); Asc(k, 5); " ["; LTrim$(k); "]"; Val(k); Len(Str$(Len(k)))
' A long one.
Dim big As String * 100000
Print "100000:"; Len(big); Asc(big, 100000)
big = "a"
Print "100000 after store:"; Asc(big, 1); Asc(big, 2); Asc(big, 100000)
' Members.
Type rec
    id As Long
    nm As String * 6
    z As Integer
End Type
Dim r As rec
Print "member initial:"; Len(r); Asc(r.nm, 1); Asc(r.nm, 6); Len(r.nm)
r.nm = "bob": r.z = 7: r.id = 1
Print "member: ["; r.nm; "]"; Asc(r.nm, 6); r.z; r.id
r.nm = "abcdefgh"
Print "member cut: ["; r.nm; "]"; r.z
' Arrays.
Dim a(2) As String * 3
a(1) = "x"
Print "array:"; Asc(a(0), 1); " ["; a(1); "]"; Len(a(1))
a(2) = "abcdef"
Print "array cut: ["; a(2); "] ["; a(1); "]"
a(3) = "bad"
Print "after a bad index: ["; a(2); "]"
Dim ra(1) As rec
ra(1).nm = "q": ra(1).z = 5
Print "member of element: ["; ra(1).nm; "]"; Asc(ra(0).nm, 1); ra(1).z
' Passed to a STRING parameter: by reference, cut and padded on the way back.
Dim f5 As String * 5
f5 = "ab": setlong f5
Print "var: ["; f5; "]"
f5 = "abcde": setshort f5
Print "var, shorter: ["; f5; "]"
f5 = "ab": showlen f5
f5 = "ab": setlong (f5)
Print "(var): ["; f5; "]"
Type rec5
    id As Long
    nm As String * 5
End Type
Dim r5 As rec5
r5.nm = "ab": setlong r5.nm
Print "member: ["; r5.nm; "]"
r5.nm = "ab": setlong (r5.nm)
Print "(member): ["; r5.nm; "]"
Dim a5(2) As String * 5
a5(1) = "ab": setlong a5(1)
Print "element: ["; a5(1); "]"
a5(1) = "ab": setlong (a5(1))
Print "(element): ["; a5(1); "]"
' Storage classes.
Dim Shared sh As String * 3
sh = "shared"
loc
loc
st
st
System

h:
Print "[handler"; Err; "]"
Resume Next

Sub setlong (t As String)
    t = "longer text"
End Sub

Sub setshort (t As String)
    t = "x"
End Sub

Sub showlen (t As String)
    Print "STRING param sees: ["; t; "]"; Len(t)
End Sub

Sub loc
    Dim l As String * 3
    Print "local:"; Asc(l, 1); " shared: ["; sh; "]"
    l = "zz"
End Sub

Sub st
    Static t As String * 3
    Print "static:"; Asc(t, 1)
    t = "s"
End Sub
