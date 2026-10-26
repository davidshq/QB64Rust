$Console:Only
' Slice (m3-mem task 4.2): runtime error 307, _MEMFREE of a block freed already, under ON ERROR:
' it is not trapped, the program stops (verification\v32_d25_free_twice).
On Error GoTo handler
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, a As Long, o As _Offset
Print "before"
m = _MemNew(8): _MemFree m
_MemFree m
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Sub keep
    Dim l As Long
    gm = _Mem(l)
End Sub
