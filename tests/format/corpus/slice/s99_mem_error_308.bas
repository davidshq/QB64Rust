$Console:Only
' Slice (m3-mem task 4.2): runtime error 308, a _MEMPUT into freed memory, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d01_put_after_free).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
m = _MemNew(8): _MemFree m
_MemPut m, m.OFFSET, 1 As Long
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
