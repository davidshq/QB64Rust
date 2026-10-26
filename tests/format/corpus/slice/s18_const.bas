$Console:Only
' CONST (spec language/constants, "CONST visibility", "CONST values"): main constants seen in later SUBs, a SUB
' constant shadowing a main constant and reusing a main variable's name, constants defined whatever the control
' flow around their line (skipped IF block, single-line IF, FOR body), the same CONST twice with an equal value,
' a plain constant used with a numeric suffix; typing (integer results and integer-valued floats are _INTEGER64,
' other floats DOUBLE), suffixes rounding half to even, ^ right-associative, the evaluator's operators, &HFFFF,
' string constants, a constant from constants, a plain constant stored in an INTEGER.
' `x * 1000000000` tells a 32-bit computation (wraps) from a 64-bit one; 2^62 tells _INTEGER64 from DOUBLE.
Const c1 = 5
Print "c1:"; c1
Const k = 1
v = 7
late
Print "main k after late:"; k; " v:"; v
Print "literal 3 * 1000000000:"; 3 * 1000000000
Const i3 = 3, f2 = 4 / 2
Print "i3, f2:"; i3 * 1000000000; f2 * 4611686018427387904
Const h2 = 2.5 * 2, sm = 2.5
Print "h2 = 2.5 * 2:"; h2 * 4611686018427387904; " sm = 2.5:"; sm * 4611686018427387904
Const l3 = 3000000000
Print "l3:"; l3; l3 * 4
Const c3 = 1 / 3, c3s! = 1 / 3, c3d# = 1 / 3
c = 1 / 3
Print "1 / 3:"; c3; c3s!; c3d#; c
Const tenth = 0.1, e30 = 1E+30, e10 = 2.5E+10
Print "floats:"; tenth; tenth * 3; tenth / 3; e30; e10
Const cmp = 3 > 2, cmpf = 2 > 3
Print "comparisons:"; cmp; cmpf
Const lg = 6 And 3, nt = Not 0, lf = 1.5 And 3, xr = 5 Xor 3, eq = 5 Eqv 3, im = 5 Imp 3, orr = 4 Or 1
Print "logic:"; lg; nt; lf; xr; eq; im; orr
Const p = 2 ^ 0.5
Print "2 ^ 0.5:"; p
Const pw = 2 ^ 3 ^ 2, pw2 = (2 ^ 3) ^ 2, pw3 = 2 ^ 10
Print "power:"; pw; pw2; pw3; " at run time:"; 2 ^ 3 ^ 2
Const rnd1% = 3.7, rh% = 2.5, rh2% = 3.5, rh3& = -2.5, rh4% = 2.4999
Print "suffixes round:"; rnd1%; rh%; rh2%; rh3&; rh4%
Const neg = -5, sub2 = 2 - 3
Print "negatives:"; neg; sub2
Const s = "x" + "y", s2$ = "z"
Print "strings: "; s; s2$
Const ref = c1 * 2
Print "c1 * 2:"; ref
Const idv = 7 \ 2, md = -7 Mod 3, fl = 7.5 \ 2, fm = 7.5 Mod 2, idn = -7 \ 2
Print "\ and MOD:"; idv; md; fl; fm; idn
Const big2 = 2147483647 + 1, i16 = 32767 + 1, mx = 9223372036854775807, mul = 3000000000 * 4
Print "64-bit:"; big2; i16; mx; mul
Const hx = &HFF, hx2 = &HFFFF, hx3 = &HFFFFFFFF
Print "hex:"; hx; hx2; hx3
Const pr = 1 + 2 * 3, pr2 = (1 + 2) * 3, pr3 = -2 ^ 2, pr4 = 10 - 2 - 3
Print "precedence:"; pr; pr2; pr3; pr4
Print "with a numeric suffix:"; c1%; c1&; c1!; c1#
Const c1 = 5
Print "the same CONST again:"; c1
If 1 Then Const inif = 4
Print "in a single-line IF:"; inif
For j = 1 To 2: Const infor = 6: Next
Print "in a FOR body:"; infor
If 0 Then
    Const skipped = 8
End If
Print "in a skipped IF block:"; skipped
Const big = 40000, half = 2.5
Dim i As Integer
i = big
Print "stored in an INTEGER:"; i;
i = half
Print i
System

Sub late
    Print "late sees c1:"; c1
    Const k = 2
    Print "late's k:"; k
    Const v = 9
    Print "late's v:"; v
End Sub
