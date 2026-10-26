$Console:Only
' Runtime comparison: SELECT CASE 1 TO 5
Dim n As Long
n = 3
Select Case n
    Case 1 To 5
        Print "in range"
    Case Else
        Print "else"
End Select
n = 10
Select Case n
    Case 1 To 5
        Print "in range"
    Case Else
        Print "else"
End Select
End
