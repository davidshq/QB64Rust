$Console:Only
' Slice (m3-mem task 4.2): runtime error 309, a _MEMPUT through a block that was never set, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d04_put_uninitialised).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
_MemPut m, 0, 1 As Long
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
