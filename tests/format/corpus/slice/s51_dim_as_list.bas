$Console:Only
' The type before the names (spec language/arrays-and-types, "DIM AS type lists"): DIM AS type with scalars and
' static arrays in one list, DIM SHARED AS, STATIC AS and SHARED AS in a SUB; _UNSIGNED, STRING, STRING * n and
' user types; every name typed as `name AS type` types it (found by its plain name under DEFINT A-Z, and by the
' type's suffix); a list that starts with an array.
Type rec
    n As Long
End Type
DefInt A-Z
Dim As Long a, b(3)
a = 100000: b(3) = a
Print "1. DIM AS LONG a, b(3):"; a; b(3); Len(a); Len(b(1)); UBound(b)
Dim As _Unsigned _Byte u1, u2
u1 = -1: u2 = 300
Print "2. DIM AS _UNSIGNED _BYTE u1, u2:"; u1; u2
Dim As String * 4 f1, f2(2)
f1 = "abcdef": f2(1) = "x"
Print "3. DIM AS STRING * 4 f1, f2(2): ["; f1; "] ["; f2(1); "]"; Len(f2(2))
Dim As String s1, s2
s1 = "one": s2 = s1 + "!"
Print "4. DIM AS STRING s1, s2: "; s1; " "; s2
Dim As rec r1, r2(2)
r1.n = 5: r2(2).n = 6
Print "5. DIM AS rec r1, r2(2):"; r1.n; r2(2).n
Dim Shared As Double g1, g2(1 To 2)
g1 = 1# / 3: g2(2) = 2.5
Print "6. DIM SHARED AS DOUBLE g1, g2(1 TO 2), in a SUB:";
show
keep
Print
keep
Print
Print "8. the names with their suffix: a&, u1~%%, s1$:"; a&; u1~%%; " "; s1$
Dim As Integer c(1 To 2), d(2, 2), e
c(2) = 2.5: d(2, 2) = 8: e = 3.5
Print "9. DIM AS INTEGER c(1 TO 2), d(2, 2), e:"; c(2); d(2, 2); e; Len(c(1)); LBound(c)
System
Sub show
    Print g1; g2(2)
End Sub
Sub keep
    Static As Long k1, k2
    Shared As Long a
    k1 = k1 + 1: k2 = k2 + a
    Print "7. STATIC AS LONG k1, k2 and SHARED AS LONG a:"; k1; k2;
End Sub
