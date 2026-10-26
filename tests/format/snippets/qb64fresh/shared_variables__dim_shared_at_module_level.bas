Dim Shared globalVar As Long
globalVar = 42
Call PrintGlobal
End

Sub PrintGlobal
    Shared globalVar
    Print globalVar
End Sub
