Option _Explicit
DefLng A-Z
$Console:Only
Dim Debug As Long
Debug = -1

'$Include:'../../../source/global/constants.bas'
'$Include:'../../../source/utilities/type.bi'
sp = "@" ' Makes the output readable

Dim i As Long, value As String, typ As Long

typ = elementGetStringValue&(createElementString$("foobar"), value)
Print "foobar element: " + value

For i = 1 To 30
    ' Skip newline characters
    If i >= 10 And i <= 13 Then _Continue
    typ = elementGetStringValue&(createElementString$("foobar" + Chr$(i) + "baz"), value)
    Print "foobar element"; i
    Print value
Next

For i = 126 To 255
    typ = elementGetStringValue&(createElementString$("foobar" + Chr$(i) + "baz"), value)
    Print "foobar element"; i
    Print value
Next

Dim s$

For i = 0 To 255
    s$ = s$ + Chr$(i)
Next

typ = elementGetStringValue&(createElementString$(s$), value)


Print "all chars: ";
For i = 1 To Len(value)
    Print Hex$(Asc(value, i)); " ";
Next

System

'$Include:'../../../source/utilities/elements.bas'
