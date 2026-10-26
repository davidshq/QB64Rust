$Console:Only
' Runtime comparison: STATIC in SUB
Call count
Call count
Call count
End

Sub count
    Static n As Long
    n = n + 1
    Print "count:"; n
End Sub
