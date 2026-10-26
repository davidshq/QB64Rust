$Console:Only
' Slice (m3-mem task 4.2): runtime error 310, _MEMCOPY from a block that was never set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d21_copy_source_uninitialised).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
n = _MemNew(8)
_MemCopy m, 0, 4 To n, n.OFFSET
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
