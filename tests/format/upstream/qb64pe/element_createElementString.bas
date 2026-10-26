Option _Explicit
DefLng A-Z
$Console:Only
Dim Debug As Long
Debug = -1

'$Include:'../../../source/global/constants.bas'
'$Include:'../../../source/utilities/type.bi'
sp = "@" ' Makes the output readable

Dim i As Long

Print "foobar element: " + createElementString$("foobar")

For i = 0 To 30
    Print "foobar element"; i; ": " + createElementString$("foobar" + Chr$(i) + "baz")
Next

For i = 126 To 255
    Print "foobar element"; i; ": " + createElementString$("foobar" + Chr$(i) + "baz")
Next

Dim s$

For i = 0 To 255
    s$ = s$ + Chr$(i)
Next
Print "all chars: " + createElementString$(s$)

System

'$Include:'../../../source/utilities/elements.bas'
