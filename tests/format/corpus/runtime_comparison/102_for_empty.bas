$Console:Only
' Runtime comparison: FOR with start > end (no STEP)
Dim i As Long
For i = 5 To 1
    Print "never"; i
Next i
Print "done"
End
