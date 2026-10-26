$Console:Only

Print HexString$(_Deflate$("this is a test"))
System

Function HexString$ (s As String)
    Dim ret As String

    For i = 1 To Len(s)
        Dim h As String
        h = Hex$(Asc(s, i))
        If Len(h) = 1 Then h = "0" + h
        ret = ret + h
    Next

    HexString$ = ret
End Function
