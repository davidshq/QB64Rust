$Console:Only
Const glob = 60

foo
baz

System

' SUB/FUNCTIONs should be able to define their on CONST that are local to that SUB/FUNCTION
' They should also be able to access the global CONSTs
Sub foo ()
    Const a = 20
    Const bar = a + 20 + glob

    Print bar
End Sub

' Separate SUB/FUNCTIONs should be able to define CONST values with the same names
Sub baz ()
    Const a = 40
    Const bar = a + 20 + glob

    Print bar
End Sub
