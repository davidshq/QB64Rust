$Console:Only
' The number of dimensions of an array parameter is not specified by the
' definition, rather the compiler works it out from the call sites. When arrays
' are handed down a chain of SUBs, the count can only travel one link per
' compile pass: The code is read top to bottom and we only keep track of
' dimension counts we know of, not the dependency tree between array arguments.
'
' A 3-deep chain therefore needs several passes. This check ensures the array
' does get resolved to the correct dimension count (if it doesn't then it will
' fail at runtime).
Dim two(1 To 2, 1 To 3)
Dim three(1 To 2, 1 To 3, 1 To 4)

level3 two()
deep3 three()
System

Sub innermost (k())
    Print "innermost dims:"; UBound(k, 1); UBound(k, 2)
End Sub

Sub middle (m())
    innermost m()
End Sub

Sub level3 (a())
    middle a()
End Sub

Sub deep3innermost (k3())
    Print "deep3 dims:"; UBound(k3, 1); UBound(k3, 2); UBound(k3, 3)
End Sub

Sub deep3middle (m3())
    deep3innermost m3()
End Sub

Sub deep3 (a3())
    deep3middle a3()
End Sub
