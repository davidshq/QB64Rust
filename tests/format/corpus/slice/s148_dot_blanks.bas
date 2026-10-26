$Console:Only
' Blanks around the dot after a plain name (m4-devices-continue-paths, design D2; verification\v19_dot_blanks_*,
' v47_c_dot_blanks): read as the name without them, a member of a TYPE variable or a plain dotted name; ERASE of a
' member array written so (as upstream arrays/04_float_family), its bounds, an element, a member passed by
' reference, nested members, a plain dotted array.
Type inner
    v As Long
End Type
Type t
    s(1 TO 3) As Long
    n As Long
    w As inner
End Type
Dim a As t
Dim p.r(2) As Long
a.s(2) = 7
Print a.s(2); a.s(2); LBound(a.s); UBound(a.s)
Erase a.S
Print a.s(2)
a.n = 3
bump a.n
Print a.n
a.w.v = 9
Print a.w.v; a.w.v
p.q = 4
Print p.q; p.q
p.r(1) = 6
Print p.r(1); UBound(p.r)
Erase p.r
Print p.r(1)
System

Sub bump (x As Long)
    x = x + 1
End Sub
