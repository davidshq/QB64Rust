$Console:Only
' Runtime comparison: EXIT SELECT (if supported)
Dim n As Long
n = 2
Select Case n
    Case 1
        Print "one"
    Case 2
        Print "two"
        Exit Select
    Case Else
        Print "other"
End Select
Print "done"
End
