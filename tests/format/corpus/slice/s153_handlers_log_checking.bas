$Console:Only
' Slice (m4-handlers-logging-checking task 4.2): the handler chain (_NEWHANDLER in a SUB, _LASTHANDLER back), the
' four log statements and _LOGMINLEVEL (the runner logs to a file, so it is 2; the entries are not compared here,
' verification\v50_b_log compares them), and $CHECKING:OFF around an error, an index and a _MEMGET.
Print "min level"; _LogMinLevel
_LogTrace "trace"
_LogInfo "info"
_LogWarn "warn"
_LogError "error"
On Error GoTo outer
Error 5
inner_test
Error 6
Dim a(1 To 3) As Long, m As _MEM, i As Long
m = _Mem(a())
$Checking:Off
For i = 1 To 3: a(i) = i * 7: Next
_MemPut m, m.OFFSET + 4, 99 As Long
Error 7
Print "not printed while the error is pending"
$Checking:On
Print "the error is serviced here, and RESUME NEXT goes on after this line"
Print "checked"; a(1); a(2); a(3); _MemGet(m, m.OFFSET + 8, Long)
_MemFree m
System

outer:
Print "outer"; Err
Resume Next
inner:
Print "inner"; Err
Resume Next

Sub inner_test
    On Error GoTo _NewHandler inner
    Error 9
    On Error GoTo _LastHandler
End Sub
