$Console:Only
' Slice (m3-asserts-debug-zlib task 2.2): $ASSERTS on the last line turns on every _ASSERT above it. Passing
' assertions of each form, the condition and the message evaluated, then a failing one without a message under
' ON ERROR: error 314 is not trapped, the program stops (verification\v33_a01, a04, a05, a08, a14).
On Error GoTo handler
Dim k As Long, d As Double, i64 As _Integer64, s As String
$If _ASSERTS_ Then
    Print "asserts on"
$End If
k = 1: d = .6: i64 = 4294967297: s = "never"
_Assert k
_Assert d, s
_Assert i64
_Assert k = 1, "m"
_Assert (k)
Call _Assert(k)
Call _Assert(k, s)
_Assert f(1), g$("message")
Print "passed"
_Assert k = 2
Print "after"
System
handler:
Print "handler"; Err
Resume Next
Function f (x)
    Print "f called"
    f = x
End Function
Function g$ (s$)
    Print "g called"
    g$ = s$
End Function
$Asserts
