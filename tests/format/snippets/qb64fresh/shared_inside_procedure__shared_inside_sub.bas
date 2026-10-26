Dim globalvar As Integer
globalvar = 100

Sub MySub
    Shared globalvar
    globalvar = globalvar + 1
End Sub
