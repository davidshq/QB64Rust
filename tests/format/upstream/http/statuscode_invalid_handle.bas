$Console:Only
On Error GoTo errorhand

h& = _OpenHost("TCP/IP:50000")

' Error, invalid handle type
Print _StatusCode(h&)

' Errors, invalid handle numbers
Print _StatusCode(0)
Print _StatusCode(20)

System

errorhand:
Print "Error:"; Err; ", Line:"; _ErrorLine
Resume Next
