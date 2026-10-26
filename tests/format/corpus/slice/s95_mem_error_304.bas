$Console:Only
' Slice (m3-mem task 4.2): runtime error 304, _MEMCOPY with both beyond their blocks, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d17_copy_both_range).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
m = _MemNew(8): n = _MemNew(8)
_MemCopy m, m.OFFSET, 9 To n, n.OFFSET
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
