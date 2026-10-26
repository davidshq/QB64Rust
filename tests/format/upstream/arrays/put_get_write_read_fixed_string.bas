$Console:Only

ReDim a(10) As String * 20

For bb = 0 To 20
    c$ = c$ + Chr$(100 + bb)
Next bb

For b = 0 To 10
    a(b) = c$
Next

ff = FreeFile
Open "test" For Binary As ff
Put ff, , a()
Close ff
Erase a
ReDim a(10) As String * 20
Close ff

Open "test" For Binary As ff
Get ff, , a()
Close ff

For f = 0 To 10
    Print a(f)
Next
If _FileExists("test") Then Kill "test"
System
