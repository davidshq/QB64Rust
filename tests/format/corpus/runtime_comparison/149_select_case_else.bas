$Console:Only
' Runtime comparison: SELECT CASE with CASE ELSE (same as 20, extra CASE ELSE coverage)
Dim n As Long
n = 1
Select Case n
    Case 1
        Print "one"
    Case 2
        Print "two"
    Case Else
        Print "other"
End Select
n = 99
Select Case n
    Case 1
        Print "one"
    Case 2
        Print "two"
    Case Else
        Print "other"
End Select
End
