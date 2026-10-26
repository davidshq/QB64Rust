$Console:Only
' Runtime comparison: ERASE on array of UDT
Type E
    n As Long
End Type
Dim arr(1 To 2) As E
arr(1).n = 10
arr(2).n = 20
Print arr(1).n; arr(2).n
Erase arr
Print "after erase:"; arr(1).n; arr(2).n
End
