$Console:Only
' Runtime comparison: Deeply nested IF/FOR/SELECT
Dim i As Long, j As Long, k As Long
For i = 1 To 2
  For j = 1 To 2
    For k = 1 To 2
      If i = 1 Then
        If j = 1 Then
          If k = 1 Then
            Print "111"
          Else
            Print "112"
          End If
        Else
          Print "12x"
        End If
      Else
        Print "2xx"
      End If
    Next k
  Next j
Next i
Print "done"
End
