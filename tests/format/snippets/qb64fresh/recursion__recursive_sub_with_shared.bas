Dim total As Long
total = 0
Call Accumulate(5)
Print total
End

Sub Accumulate (n As Long)
    Shared total
    If n > 0 Then
        total = total + n
        Call Accumulate(n - 1)
    End If
End Sub
