$Console:Only
' Runtime comparison: SELECT CASE
Dim n As Long
n = 1
Select Case n
    Case 1
        Print "case 1"
    Case 2
        Print "case 2"
    Case Else
        Print "else"
End Select
n = 2
Select Case n
    Case 1
        Print "case 1"
    Case 2
        Print "case 2"
    Case Else
        Print "else"
End Select
n = 9
Select Case n
    Case 1
        Print "case 1"
    Case 2
        Print "case 2"
    Case Else
        Print "else"
End Select
End
