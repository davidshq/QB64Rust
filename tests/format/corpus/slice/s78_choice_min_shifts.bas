$Console:Only
' Slice (m3-console-files, task 2.5; from verification\v30_g_iif_min_max, v30_h_shifts): _IIF, _MIN, _MAX, _CLAMP,
' _SHL, _SHR, _ROL and _ROR, 24 numbered lines. _IIF evaluates only the part it chooses (a FUNCTION that prints, an
' element outside its array, a division by zero); its condition is rounded; the result of the four is typed by a
' rule of their own (a SINGLE beside an _INTEGER64, signed beside unsigned of one width); the rotations turn within
' the width of their first argument's type.
On Error GoTo handler
Type rec
    s As Single
    q As _Integer64
End Type
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer, l As Long, ul As _Unsigned Long
Dim q As _Integer64, uq As _Unsigned _Integer64, s As Single, d As Double, f As _Float, x As String * 5
Dim v As String, a(3) As Long, n As Long, r As rec, bt As _Bit, nan As Single
Print "1:"; _IIf(1, 10, 20); _IIf(0, 10, 20); _IIf(-1, "yes", "no"); _IIf(0, "yes", "no")
Print "2:"; _IIf(1, pa&, pb&); _IIf(0, pa&, pb&)
n = 9: a(1) = 5
Print "3:"; _IIf(n <= 3, a(n), -1); _IIf(n > 3, -1, a(n))
n = 1
Print "4:"; _IIf(n <= 3, a(n), -1); _IIf(n, 100 \ n, 0)
n = 0
Print "5:"; _IIf(n, 100 \ n, 0); _IIf(n = 0, 0, 100 Mod n)
Print "6: an element outside in the part chosen:";: l = _IIf(1, a(9), 2): Print l
Print "7:"; _IIf(0.5, "t", "f"); _IIf(0.6, "t", "f"); _IIf(0.4, "t", "f"); _IIf(-0.5, "t", "f"); _IIf(1.5, "t", "f");
d = 1E-30: s = 2.5: q = 4294967296
Print _IIf(d, "t", "f"); _IIf(s, "t", "f"); _IIf(q, "t", "f"); _IIf(2.5 - s, "t", "f")
x = "ab": v = "abc"
Print "8:"; Len(_IIf(0, x, v)); Len(_IIf(1, x, v)); "["; _IIf(1, x, v); "]["; _IIf(0, x + "!", Mid$(v, 2)); "]"
i = 7: l = 100000: s = 1.5
Print "9:"; _IIf(1, i, l); _IIf(0, i, l); _IIf(1, i, s); _IIf(0, i, s) * 2; _IIf(1, 1, 2.5) / 2
Print "10:"; _IIf(_IIf(n, 0, 1), "inner 1", "inner 0"); _IIf(1, _IIf(0, 1, 2), 3)
v = _IIf(Len(v) > 2, v + v, "")
Print "11: "; v; _IIf(pa& > 0 And pb& > 0, " both", " not both")
Print "12:"; _Min(3, 7); _Max(3, 7); _Min(-3, -7); _Max(-3, -7); _Min(2.5, 2); _Max(2.5, 2); _Min(1E+30, 1D+300)
b = -1: ub = 255: i = -1: ui = 65535: l = -1: ul = 4294967295
Print "13:"; _Min(b, ub); _Max(b, ub); _Min(ub, b); _Min(i, ui); _Max(l, ul); _Min(i, ub); _Max(b, ui)
s = 16777217: q = 16777217: d = 16777217
Print "14:"; _Max(s, q); _Max(q, s); _Min(q, 16777218); _Max(d, q); _Max(q, 0.5)
uq = 0: uq = uq - 1: q = -1
Print "15:"; _Min(uq, q); _Max(q, uq); _Min(q, 5); _Max(uq, 5)
r.s = 3.5: r.q = 3
Print "16:"; _Max(r.s, r.q); _Min(r.s, r.q); _Min(a(1), 4.5); _Max(bt, 1); _Min(bt, bt)
Print "17:"; _Min(pa&, pb&); _Max(pb&, pa&); _Clamp(pa&, pb&, pc&)
Print "18:"; _Clamp(15, 0, 10); _Clamp(-5, 0, 10); _Clamp(5, 0, 10); _Clamp(5, 10, 0); _Clamp(2.75, 1, 2.5);
Print _Clamp(i, ub, 300); _Clamp(ub, 1, 10)
nan = 0: nan = nan / nan
Print "19:"; _Min(nan, 1) = _Min(nan, 1); _Min(1, nan); _Max(1, nan)
l = _Min(i, b) + _Max(l, 5) * 2
Print "20:"; l; Hex$(_Min(i, b)); " "; Hex$(_Max(ub, 1)); Str$(_Min(2, 3)); _Min(_Max(1, 2), _Min(3, 4))
Print "21:"; _ShL(1, 4); _ShR(256, 4); _ShL(1, 63); _ShR(-1, 1); _ShL(1, 64); _ShL(1, 65); _ShR(8, -1); _ShL(2.5, 1.5);
i = -2: Print _ShR(i, 1); _ShL(i, 1)
b = -127: ub = 129: i = 1: l = 1: q = 1
Print "22:"; _RoL(ub, 1); _RoR(ub, 1); _RoL(b, 1); _RoR(i, 1); _RoL(i, 15); _RoR(l, 1); _RoR(q, 1)
Print "23:"; _RoL(1, 16); _RoR(1, 1); _RoL(70000, 31); _RoR(70000, 32); _RoL(5000000000, 1); _RoL(-1, 3); _RoL(i + 0, 63)
Print "24:"; _RoL(ub, 9); _RoL(ub, -1); _RoR(ub, 8); _RoL(i, q + 16); Hex$(_RoL(b, 4)); " "; _RoL(_RoL(ub, 4), 4)
System
handler:
Print " <error"; Err; ">";
Resume Next
Function pa&
    Print " (a)";
    pa& = 5
End Function
Function pb&
    Print " (b)";
    pb& = 1
End Function
Function pc&
    Print " (c)";
    pc& = 3
End Function
