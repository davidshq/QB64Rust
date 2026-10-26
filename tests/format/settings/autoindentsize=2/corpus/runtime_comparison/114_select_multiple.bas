$Console:Only
' Runtime comparison: SELECT CASE multiple CASE
Dim n As Long
n = 1
Select Case n
  Case 1, 2, 3
    Print "one two three"
  Case 4, 5
    Print "four five"
  Case Else
    Print "else"
End Select
n = 4
Select Case n
  Case 1, 2, 3
    Print "one two three"
  Case 4, 5
    Print "four five"
  Case Else
    Print "else"
End Select
End
