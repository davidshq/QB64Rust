$Console:Only
' Runtime comparison: EXIT FOR
Dim i As Long
For i = 1 To 10
    Print i;
    If i = 3 Then Exit For
Next i
Print
Print "done"
End
