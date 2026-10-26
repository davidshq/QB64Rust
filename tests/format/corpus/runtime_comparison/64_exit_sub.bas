$Console:Only
' Runtime comparison: EXIT SUB
Call mySub
Print "back"
End

Sub mySub
    Print "in sub"
    Exit Sub
    Print "never"
End Sub
