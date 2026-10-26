$Console:Only
'test for writing and reading records to file using static or dynamic nested array. Condition for this is, that string array must have fixed size.
$Unstable:TypeFields
FF = FreeFile

'static nested array
Type TestStatic
    _Static text(9) As String * 23
    _Dynamic nr(9) As _Unsigned Long
    something As String * 7
End Type

ReDim a(1) As TestStatic
rec$ = "Static Nested Record"

For g = 0 To 1
    For f = 0 To 9
        a(g).text(f) = rec$ + Str$(f + 1)
        a(g).nr(f) = g * 10 + f + 1
    Next f
    a(g).something = "padding"
Next g
If _FileExists("test") Then Kill "test"
Open "test" For Binary As FF
Put FF, , a() '            write content
ReDim a(1) As TestStatic ' erase content from array

'? "empty control:"
'FOR g = 0 TO 1
'FOR f = 0 TO 9
'? a(g).text(f); a(g).nr(f); a(g).something
'NEXT f, g
'SLEEP    '                is empty

Get FF, 1, a() '           read content from file
Close FF

For g = 0 To 1
    For f = 0 To 9
        Print a(g).text(f); a(g).nr(f); a(g).something
Next f, g

If _FileExists("test") Then Kill "test"
System

'is possible using GET / PUT for UDT and normal arrays - but STRING must have fixed size!
