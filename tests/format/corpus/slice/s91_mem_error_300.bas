$Console:Only
' Slice (m3-mem task 4.2): runtime error 300, a _MEMPUT that reaches beyond the block, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d02_put_out_of_range).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
m = _Mem(a)
_MemPut m, m.OFFSET + 2, 1 As Long
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
