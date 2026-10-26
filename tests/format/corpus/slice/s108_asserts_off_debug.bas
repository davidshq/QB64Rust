$Console:Only
' Slice (m3-asserts-debug-zlib task 2.2): without $ASSERTS an _ASSERT checks nothing, but its condition and message
' are evaluated (verification\v33_a03). $DEBUG on the last line sets _DEBUG_ and _SOCKETS_ above it and changes
' nothing else outside the IDE (v33_d01, d11).
$If _ASSERTS_ Then
    PRINT "asserts on"
$Else
    Print "asserts off"
$End If
$If _DEBUG_ Then
    Print "debug on"
$End If
$If _SOCKETS_ Then
    Print "sockets on"
$End If
_Assert f(0), g$("never")
_Assert 0
_Assert 0, "never"
Print "after"
System
Function f (x)
    Print "f called"
    f = x
End Function
Function g$ (s$)
    Print "g called"
    g$ = s$
End Function
$Debug
