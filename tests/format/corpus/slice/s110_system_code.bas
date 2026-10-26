$Console:Only
' Slice (m3-asserts-debug-zlib task 4.1): SYSTEM with an exit code (verification\v33_c05, c22, c23). The code is
' a FUNCTION that raises an error under ON ERROR and gives 2.7: the handler runs first, then the code is stored,
' rounded through qbr_float_to_long (3), from inside a SUB.
On Error GoTo handler
Print "before"
s
Print "not reached"
System
handler:
Print "handler"; Err
Resume Next
Sub s
    Print "in s"
    System f#
End Sub
Function f#
    Error 5
    f# = 2.7
End Function
