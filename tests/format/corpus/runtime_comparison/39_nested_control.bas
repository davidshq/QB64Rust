$Console:Only
' Runtime comparison: nested IF and FOR
Dim i As Long, j As Long
For i = 1 To 2
    If i = 1 Then
        For j = 1 To 2
            Print "a"; j;
        Next j
    Else
        Print "b";
    End If
Next i
Print
End
