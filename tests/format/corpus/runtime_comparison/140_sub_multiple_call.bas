$Console:Only
' Runtime comparison: same SUB called twice
Call p
Call p
End

Sub p
    Print "p"
End Sub
