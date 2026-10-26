$Console:Only
' Default types in the main module (spec language/default-types): DEFINT, DEFLNG, DEFSNG, DEFDBL, DEFSTR with
' ranges, single letters, any letter case, a range written backwards and the underscore entry; a name before and
' after a statement (the plain name is the name plus the default at its line), a variable declared with AS, a
' later statement for the same letter, a statement that never runs (skipped by GOTO, in an IF block); what the
' default types: DIM, DIM SHARED, an array, a FOR variable, but not a CONST; _DEFINE with every type a suffix
' can give, STRING * n and _BIT * n included. LEN of a variable shows its type.
a = 1.5
DefInt A-Z
b = 1.5
Print "1. a = 1.5 above DEFINT A-Z, b = 1.5 below: a, b, a!"; a; b; a!
x = 2.5
Print "2. x = 2.5: x, x / 2, LEN(x)"; x; x / 2; Len(x)
Dim dd As Double
dd = 1.5: s! = 2.5: n$ = "q"
Print "3. AS and suffixes win: "; dd; s!; n$; Len(dd); Len(s!)
Dim lg As Long
lg = 100000
DefStr L
Print "4. DIM lg AS LONG, then DEFSTR L: lg, LEN(lg)"; lg; Len(lg)
DefInt L
DefDbl D
i = 1.5: d = 1.5
Print "5. DEFDBL D below DEFINT A-Z: i, d"; i; d
DefSng A-Z
DefLng A-C, X
b3 = 100000: d3 = 1.5: x3 = 70000: c3 = 2.5
Print "6. DEFLNG a-c, x: b3, d3, x3, c3"; b3; d3; x3; c3
DefSng A-Z
DefStr S
s = "text"
Print "7. DEFSTR S: "; s + "!"; Len(s)
DefSng A-Z
GoTo skip
DefLng G
skip:
g = 2.5
Print "8. DEFLNG G skipped by GOTO: g, LEN(g)"; g; Len(g)
If 0 Then
    DefDbl H
End If
h = 1# / 3
Print "9. DEFDBL H in an IF block that does not run: h"; h
DefSng A-Z
DefInt Z-W
v4 = 2.5: w4 = 2.5: x4 = 2.5: y4 = 2.5: z4 = 2.5
Print "10. DEFINT z-w: v4 to z4"; v4; w4; x4; y4; z4
DefSng A-Z
DefInt K, K, K-M, L
k5 = 2.5: l5 = 2.5: m5 = 2.5: n5 = 2.5
Print "11. defint k, K, k-m, l: k5 to n5"; k5; l5; m5; n5
DefInt A-Z
__a = 2.5
Print "12. __a under DEFINT A-Z:"; __a; Len(__a)
DefInt _, Y-Z
__b = 2.5
DefDbl Y-_, Y
__c = 2.5: y6 = 2.5: z6 = 2.5: x6 = 2.5
Print "    after DEFINT _, y-z: __b; after DEFDBL y-_, y: __c, y6, z6, x6"; __b; __c; y6; z6; x6
DefSng _, A-Z
DefLng A-Z: DefInt Q: q7 = 2.5: r7 = 2.5
Print "13. two statements on one line: LEN(q7), LEN(r7)"; Len(q7); Len(r7)
DefInt A-Z
Dim t(3), dv
Dim Shared gv
t(1) = 2.5: dv = 2.5: gv = 2.5
Print "14. DIM t(3), dv and DIM SHARED gv under DEFINT A-Z:"; t(1); dv; gv; Len(t(1)); Len(dv); Len(gv)
For fi = 1 To 2: Next
Print "15. a FOR variable:"; fi; Len(fi)
Const ck = 2.5
DefStr S
Const s1 = 5
DefInt S
Const kk = "abc"
Print "16. CONST ck = 2.5, CONST s1 = 5 under DEFSTR S, CONST kk = a string under DEFINT: "; ck; s1 + 1; kk + "!"
DefSng A-Z
_Define A As INTEGER : a1 = 2.5: Print "17. INTEGER"; a1; Len(a1)
_Define B As LONG : b1 = 2.5: Print "    LONG"; b1; Len(b1)
_Define C As SINGLE : c1 = 2.5: Print "    SINGLE"; c1; Len(c1)
_Define D As DOUBLE : d1 = 1# / 3: Print "    DOUBLE"; d1; Len(d1)
_Define E As _FLOAT : e1 = 2.5: Print "    _FLOAT"; e1; Len(e1)
_Define F As _BYTE : f1 = -1: Print "    _BYTE"; f1; Len(f1)
_Define G As _UNSIGNED _BYTE : g1 = -1: Print "    _UNSIGNED _BYTE"; g1; Len(g1)
_Define H As _UNSIGNED INTEGER : h1 = -1: Print "    _UNSIGNED INTEGER"; h1; Len(h1)
_Define I As _UNSIGNED LONG : i1 = -1: Print "    _UNSIGNED LONG"; i1; Len(i1)
_Define J As _INTEGER64 : j1 = 5000000000.5: Print "    _INTEGER64"; j1; Len(j1)
_Define K As _UNSIGNED _INTEGER64 : k1 = -1: Print "    _UNSIGNED _INTEGER64"; k1; Len(k1)
_Define L As _OFFSET : l1 = -1: Print "    _OFFSET"; l1; Len(l1)
_Define M As _UNSIGNED _OFFSET : m1 = 5: Print "    _UNSIGNED _OFFSET"; m1; Len(m1)
_Define N As STRING : n1 = "abc": Print "    STRING "; n1; Len(n1)
_Define O-Q, S As LONG 
o1 = 2.5: p1 = 2.5: q1 = 2.5: r1 = 2.5: s2 = 2.5
Print "18. _DEFINE o-q, s AS LONG: LEN of o1, p1, q1, s2, and r1"; Len(o1); Len(p1); Len(q1); Len(s2); r1
_Define U As _UNSIGNED _BYTE 
u = -1
Print "19. _Define U As _Unsigned _Byte: u"; u
_Define V As STRING * 5 
v1 = "abcdefgh"
Print "20. _DEFINE v AS STRING * 5: ["; v1; "]"; Len(v1)
_Define W As _BIT * 4 
_Define X As _UNSIGNED _BIT 
_Define Y As _BIT 
w1 = 7: Print "21. _BIT * 4, _UNSIGNED _BIT, _BIT:"; w1;
w1 = 9: x1 = 1: y1 = 1: Print w1; x1; y1
_Define _ As DOUBLE 
__d = 1# / 3
Print "22. _DEFINE _ AS DOUBLE: __d"; __d
System
