Dim x As Long
Dim y As Long
x = 5
y = 10
Call SwapValues
Print x; y
End

Sub SwapValues
    Shared x, y
    Dim temp As Long
    temp = x
    x = y
    y = temp
End Sub
