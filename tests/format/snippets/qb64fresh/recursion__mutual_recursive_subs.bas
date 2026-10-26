DECLARE SUB SubB(n AS LONG)
Call SubA(3)
End

Sub SubA (n As Long)
    If n > 0 Then
        Print "A:"; n
        Call SubB(n - 1)
    End If
End Sub

Sub SubB (n As Long)
    If n > 0 Then
        Print "B:"; n
        Call SubA(n - 1)
    End If
End Sub
