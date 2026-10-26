$Console:Only
' Runtime comparison: IF / ELSEIF / ELSE blocks, PRINT per branch
Dim x As Long
x = 1
If x = 1 Then
    Print "branch one"
ElseIf x = 2 Then
    Print "branch two"
Else
    Print "branch else"
End If
x = 2
If x = 1 Then
    Print "branch one"
ElseIf x = 2 Then
    Print "branch two"
Else
    Print "branch else"
End If
x = 3
If x = 1 Then
    Print "branch one"
ElseIf x = 2 Then
    Print "branch two"
Else
    Print "branch else"
End If
End
