$Console:Only
$Asserts
' Slice (m3-asserts-debug-zlib task 2.2): a failing _ASSERT with a message in a console program writes the message
' and raises error 315, which ON ERROR does not trap (verification\v33_a02).
On Error GoTo handler
Print "before"
_Assert 1 > 2, "bad" + " value"
Print "after"
System
handler:
Print "handler"; Err
Resume Next
