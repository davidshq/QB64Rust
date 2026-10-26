$Console:Only
' Runtime comparison: Optional parameters (if supported)
' If OPTIONAL not supported, this tests SUB with two params called with two args
Sub foo (a As Long, b As Long)
    Print "foo:"; a; b
End Sub

Call foo(1, 2)
Call foo(10, 20)
End
