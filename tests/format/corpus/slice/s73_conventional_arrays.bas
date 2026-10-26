$Console:Only
' Slice (m3-get-put-bload, task 4.3; from verification\v28_e_cmem_arrays): arrays in conventional memory used
' as any array is: ERASE, an
' array listed in COMMON (dynamic to the old compiler), REDIM, a dynamic array, passed to a SUB, LBOUND and
' UBOUND, and the order of the segments.
Common Shared cm() As Integer
Dim Shared cm(1 To 20) As Integer
Dim a(1 To 10) As Integer, b(1 To 10) As Integer, n As Integer, i As Integer
n = 8
Dim dy(1 To n) As Long
ReDim rd(1 To 4) As Integer
On Error GoTo handler
' The static ones lie first, in the order of their DIM; the dynamic ones follow from the other end.
Print VarSeg(b(1)) - VarSeg(a(1)); VarSeg(cm(1)) > VarSeg(b(1)); VarSeg(dy(1)) > VarSeg(b(1)); VarSeg(rd(1)) > VarSeg(b(1))
Print VarSeg(cm(1)) - VarSeg(dy(1)); VarSeg(dy(1)) - VarSeg(rd(1))
Print VarPtr(cm(3)); VarPtr(dy(3)); VarPtr(rd(3))
' Values through DEF SEG.
For i = 1 To 10: a(i) = i: Next
cm(1) = 258: dy(1) = 65537: rd(2) = 7
Def Seg = VarSeg(cm(1)): Print Peek(VarPtr(cm(1))); Peek(VarPtr(cm(1)) + 1)
Def Seg = VarSeg(dy(1)): Print Peek(VarPtr(dy(1))); Peek(VarPtr(dy(1)) + 2)
Def Seg = VarSeg(rd(1)): Poke VarPtr(rd(2)), 9: Print rd(2)
Def Seg
' ERASE: a static one is cleared, a dynamic one released; then allocated again.
Erase a
Print a(3); LBound(a); UBound(a)
Erase cm
Print "cm erased"
i = cm(1)
ReDim cm(1 To 5) As Integer
cm(5) = 55: Print cm(5); UBound(cm)
ReDim _Preserve rd(1 To 8) As Integer
Print rd(2); UBound(rd)
Def Seg = VarSeg(rd(1)): Print Peek(VarPtr(rd(2))): Def Seg
Erase dy
ReDim dy(1 To 3) As Long
dy(3) = 3: Print dy(3)
' Passed to a SUB.
Call sb(a(), rd())
Print a(2); rd(1); UBound(rd)
Def Seg = VarSeg(rd(1)): Print Peek(VarPtr(rd(1))): Def Seg
System
handler:
Print "error"; Err
Resume Next

Sub sb (p() As Integer, r() As Integer)
    p(2) = 22
    Erase p
    p(2) = 23
    ReDim r(1 To 12) As Integer
    r(1) = 77
End Sub
