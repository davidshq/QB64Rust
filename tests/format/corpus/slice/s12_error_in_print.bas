$Console:Only
' Errors raised inside a PRINT (spec language/error-handling, "Resume"; spec compiler/pipeline, "Error inside a
' PRINT"): RESUME re-runs the whole PRINT; RESUME NEXT skips the rest of it, line end included; an ERROR inside a
' FUNCTION called from a PRINT resumes inside the FUNCTION; an untrapped error skips the rest of its PRINT (run
' with QB64PE_NOPROMPT=continue, s12_error_in_print.noprompt, so the program goes on).
Dim k As Long
k = -1
On Error GoTo fixk
Print "a"; Chr$(k); "b"
On Error GoTo skip
Print "c"; Chr$(300); "d"
Print "e"; f&; "g"
Print "h"; fstr$("i"); Chr$(-1); "j"
Print Chr$(72); Chr$(255 - 150); Chr$(0 + 33)
On Error GoTo 0
Print "untrapped"; Chr$(-5); "not printed"
Print "after untrapped"
System

fixk:
Print "fixk"; Err
k = 65
Resume

skip:
Print "skip"; Err
Resume Next

Function f&
    f& = 1
    Error 7
    f& = 2
End Function

Function fstr$ (t As String)
    fstr$ = t + t
End Function
