$Console:Only
' SELECT CASE with numeric and IS
Dim x As Long
x = 2
Select Case x
    Case 1: Print "one"
    Case 2: Print "two"
    Case Else: Print "other"
End Select
Select Case x
    Case Is < 2: Print "lt2"
    Case Is >= 2: Print "ge2"
End Select
End
