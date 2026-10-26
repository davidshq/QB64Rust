Call PrintBinary(10)
Print
End

Sub PrintBinary (n As Long)
    If n > 0 Then
        Call PrintBinary(n \ 2)
        Print n Mod 2;
    End If
End Sub
