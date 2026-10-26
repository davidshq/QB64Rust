$Console:Only
' Slice (m3-get-put-bload, task 4.3): DEF SEG, VARSEG, VARPTR, PEEK, POKE, BSAVE and BLOAD on arrays of the main
' module (verification\v28_b_bload, v28_d_varptr_index). An array named in VARPTR or VARSEG lies in
' conventional memory: the arrays one after the other in the order of their DIM, each rounded up to 16 bytes.
' Only differences of segments are printed, never a segment itself.
Dim Shared calls As Integer
Dim a(1 To 10) As Integer, b(1 To 10) As Integer, c(0 To 4) As Long, m(1 To 3, 1 To 4) As Integer
Dim i As Integer, s As Single, l As Long, q As _Integer64
Dim Shared g(1 To 10) As Integer
On Error GoTo handler
' Offsets: the index from the lower bound times the element's size.
Print VarPtr(a(1)); VarPtr(a(2)); VarPtr(a(10)); VarPtr(c(0)); VarPtr(c(3)); VarPtr(m(1, 1)); VarPtr(m(2, 1)); VarPtr(m(1, 2))
Print VarSeg(b(1)) - VarSeg(a(1)); VarSeg(c(0)) - VarSeg(b(1)); VarSeg(m(1, 1)) - VarSeg(c(0)); VarSeg(g(1)) - VarSeg(a(1))
Print VarSeg(a(1)) = VarSeg(a(7)); VarSeg(a(1)) > 0
' An offset is an _INTEGER64 value; the index of VARPTR is evaluated and checked, that of VARSEG is not.
Print Str$(VarPtr(a(2))); VarPtr(a(2)) + 1; VarPtr(a(3)) / 4; VarPtr(a(10)) * 100000; -VarPtr(a(2))
i = VarPtr(a(3)): s = VarPtr(a(4)): q = VarPtr(a(5))
Print i; s; q
l = VarPtr(a(f%(2)))
Print "VARPTR:"; l; "calls"; calls
calls = 0
l = VarSeg(a(f%(3))) - VarSeg(a(1))
Print "VARSEG:"; l; "calls"; calls
l = VarPtr(a(11))
Print "after 11:"; l
l = VarSeg(a(99)) - VarSeg(a(1))
Print "segment of 99:"; l
' POKE and PEEK through DEF SEG.
Def Seg = VarSeg(a(1))
Poke VarPtr(a(1)), 65
Poke VarPtr(a(1)) + 1, 1
Poke VarPtr(a(3)), 255
Poke VarPtr(a(3)) + 1, 255
Print a(1); a(2); a(3)
a(4) = &H1234
Print Peek(VarPtr(a(4))); Peek(VarPtr(a(4)) + 1)
Poke VarPtr(a(5)), 300: Print a(5)
Poke VarPtr(a(5)), -1: Print a(5)
s = VarPtr(a(6)): Poke s, 7.6: Print a(6); Peek(s)
Def Seg
' BSAVE and BLOAD.
For i = 1 To 10: a(i) = i * 11: Next
Def Seg = VarSeg(a(1))
BSave "s72_bload.tmp", VarPtr(a(1)), 20
Def Seg
Open "s72_bload.tmp" For Binary As #1
Print "file length"; LOF(1)
Close #1
Def Seg = VarSeg(b(1))
BLoad "s72_bload.tmp", VarPtr(b(1))
Def Seg
For i = 1 To 10: Print b(i);: Next: Print
' With no offset: the one of the BSAVE, in the current segment.
For i = 1 To 10: a(i) = 0: Next
Def Seg = VarSeg(a(1))
BLoad "s72_bload.tmp"
Def Seg
For i = 1 To 10: Print a(i);: Next: Print
' In a procedure, on a shared array.
Call fill
For i = 1 To 10: Print g(i);: Next: Print
' A file that does not exist.
BLoad "s72_missing.tmp", VarPtr(b(1))
Print "after the missing file"
Kill "s72_bload.tmp"
System
handler:
Print "error"; Err
Resume Next

Sub fill
    Def Seg = VarSeg(g(1))
    BLoad "s72_bload.tmp", VarPtr(g(1))
    Poke VarPtr(g(10)), 9
    Def Seg
End Sub

Function f% (n As Integer)
    calls = calls + 1
    f% = n
End Function
