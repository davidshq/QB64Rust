Dim x As Integer
x = 5
Select EveryCase x
    Case 5
        Print "five"
    Case Is > 3
        Print "greater than three"
    Case Else
        Print "other"
End Select
