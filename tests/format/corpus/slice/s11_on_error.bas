$Console:Only
' Error handlers (spec language/error-handling): RESUME NEXT, RESUME label, retry reaching a second handler,
' ERR before, inside and after a handler, ERROR with unusual values, errors raised inside a SUB and a FUNCTION,
' a handler set inside a SUB, ON ERROR GOTO 0, and a critical error that is not trapped (last).
On Error GoTo handler
Print "ERR before:"; Err
Error 5
Print "after RESUME NEXT, ERR:"; Err
Error 0
Error -1
Error 2.5
Error 3.5
Error 70000
raises
Print "f:"; fraises&
On Error GoTo tolabel
Error 6
Print "skipped by RESUME back"
back: Print "at back"
On Error GoTo first
Error 7
Print "after retry"
sethandler
Error 9
On Error GoTo 0
Print "handler removed"
On Error GoTo handler
Error 11
Print "not reached: error 11 is critical"
System

handler:
Print "handler: ERR"; Err; "ERL"; Erl
Resume Next

tolabel:
Print "tolabel: ERR"; Err
Resume back

first:
Print "first: ERR"; Err
On Error GoTo second
Resume

second:
Print "second: ERR"; Err
Resume Next

subh:
Print "subh: ERR"; Err
Resume Next

Sub raises
    Print "in raises"
    Error 8
    Print "raises continues"
End Sub

Function fraises&
    fraises& = 1
    Error 7
    fraises& = 2
End Function

Sub sethandler
    On Error GoTo subh
End Sub
