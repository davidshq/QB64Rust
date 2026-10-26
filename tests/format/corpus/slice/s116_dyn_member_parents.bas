$Console:Only
$Unstable:TypeFields
' Slice (m3-typefields task 4.4): _DYNAMIC member arrays of the elements of an array and what the parent's
' statements do to them (verification\v35_b, v35_x51): each element's own member, REDIM of the parent (plain,
' _PRESERVE, _RETAIN), ERASE of a dynamic parent, a static parent and ERASE of it (its members released and left
' unallocated: error 9 until a REDIM of the member), SWAP of two elements, an element copied to another and to a
' variable and their isolation.
Type t
    n As Long
    v(1) _Dynamic As Long
End Type
On Error GoTo h
Dim i As Long, j As Long
Dim w As t
ReDim p(0 To 2) As t
Print "1. first bounds of p(1).v:"; LBound(p(1).v); UBound(p(1).v)
For i = 0 To 2
    p(i).n = i
    ReDim p(i).v(0 TO i + 1) AS LONG
    For j = 0 To i + 1: p(i).v(j) = 100 * i + j: Next
Next
GoSub show
Print "2. REDIM _PRESERVE p(0 TO 3):"
ReDim _Preserve p(0 To 3) As t
GoSub show
Print "3. REDIM _RETAIN p(1 TO 3):"
ReDim _Retain p(1 To 3) As t
GoSub show
Print "4. SWAP p(1), p(2):"
Swap p(1), p(2)
GoSub show
Print "5. p(3) = p(1), then p(1).v(0) = -1:"
p(3) = p(1)
p(1).v(0) = -1
GoSub show
Print "6. w = p(2), then p(2).v(1) = -2:"
w = p(2)
p(2).v(1) = -2
Print "   w:"; w.n; LBound(w.v); UBound(w.v); w.v(0); w.v(1)
Print "7. plain REDIM p(0 TO 1):"
ReDim p(0 To 1) As t
GoSub show
Print "8. ERASE p, then REDIM p(0 TO 0):"
Erase p
ReDim p(0 To 0) As t
GoSub show
Print "9. a static parent s(1):"
Dim s(1) As t
ReDim s(1).v(5 TO 6) AS LONG
s(1).v(6) = 66
Print "  "; LBound(s(0).v); UBound(s(0).v); LBound(s(1).v); UBound(s(1).v); s(1).v(6)
Erase s
Print "   ERASE s:"; LBound(s(1).v); UBound(s(1).v); s(1).n
Print "10. REDIM of a member after ERASE of a static parent:"
Dim e(1) As t
Erase e
ReDim e(1).v(2 TO 3) AS LONG
e(1).v(3) = 3
Print "  "; LBound(e(1).v); UBound(e(1).v); e(1).v(3)
Print "  "; LBound(e(0).v)
System
show:
For i = LBound(p) To UBound(p)
    Print "   p("; i; "): n ="; p(i).n; " v"; LBound(p(i).v); "TO"; UBound(p(i).v); ":";
    For j = LBound(p(i).v) To UBound(p(i).v): Print p(i).v(j);: Next
    Print
Next
Return
h:
Print "[error"; Err; "]";
Resume Next
