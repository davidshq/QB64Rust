$Console:Only
' Runtime comparison: SELECT CASE IS
Dim n As Long
n = 5
Select Case n
    Case Is < 3
        Print "lt 3"
    Case Is < 7
        Print "lt 7"
    Case Else
        Print "else"
End Select
n = 1
Select Case n
    Case Is < 3
        Print "lt 3"
    Case Is < 7
        Print "lt 7"
    Case Else
        Print "else"
End Select
End
