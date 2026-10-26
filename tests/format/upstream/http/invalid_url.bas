$Console:Only
On Error GoTo errorhand

' This domain isn't valid at all, an error is triggered
h& = _OpenClient("HTTP:NotARealURL")
Print h&

' Only HTTP URLs are supported, an error is triggered
h& = _OpenClient("HTTP:ftp://example.com")
Print h&

System

errorhand:
Print "Error:"; Err; ", Line:"; _ErrorLine
Resume Next
