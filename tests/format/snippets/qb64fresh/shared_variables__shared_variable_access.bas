Dim counter As Long
counter = 10
Call IncrementCounter
Print counter
End

Sub IncrementCounter
    Shared counter
    counter = counter + 1
End Sub
