$Console:Only
' Slice (m3-mem task 4.2): runtime error 301, _MEM(offset, size) with a negative size, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d10_mem_negative_size).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
o = _Offset(a)
m = _Mem(o, -1)
Print m.SIZE; m.TYPE
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
