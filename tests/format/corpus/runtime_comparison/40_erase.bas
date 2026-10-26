$Console:Only
' Runtime comparison: ERASE
Dim arr(1 To 3) As Long
arr(1) = 1
arr(2) = 2
arr(3) = 3
Print "before:"; arr(1); arr(2); arr(3)
Erase arr
ReDim arr(1 To 2) As Long
arr(1) = 10
arr(2) = 20
Print "after:"; arr(1); arr(2)
End
