$Console:Only
' Runtime comparison: SUB without CALL keyword
Dim n As Long
n = 0
mySub n
Print "after:"; n
End

Sub mySub (x As Long)
    x = 1
End Sub
