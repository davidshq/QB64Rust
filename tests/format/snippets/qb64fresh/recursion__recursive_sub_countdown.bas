Call CountdownSub(5)
End

Sub CountdownSub (n As Long)
    If n > 0 Then
        Print n
        Call CountdownSub(n - 1)
    End If
End Sub
