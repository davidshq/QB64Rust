$Console:Only
' DECLARE LIBRARY without a name (the C runtime the program's C++ already reaches) and DECLARE DYNAMIC
' LIBRARY "kernel32" (found in System32, loaded when the program starts; verification\v37_a, v37_g).
Declare Library
    Function isdigit& (ByVal c As Long)
    Function toupper& (ByVal c As Long)
    Function labs& Alias abs (ByVal v As Long)
    Function strlen~%& (s As String)
    Function atoi& (s As String)
    Function fabs# (ByVal v As Double)
End Declare
Declare Dynamic Library "kernel32"
    Function lstrlenA& (s As String)
    Function GetCurrentProcessId~&
    Sub SetLastError (ByVal e As _Unsigned Long)
    Function GetLastError~&
End Declare

Print isdigit&(Asc("2")) <> 0; isdigit&(Asc("x"))
Print Chr$(toupper&(Asc("q"))); labs&(-17); fabs#(-2.5)
Print strlen~%&("four"); atoi&("123abc")
Print "kernel32:"; lstrlenA&("seven!!"); GetCurrentProcessId~& <> 0
' Read before PRINT, whose output could set the last error.
SetLastError 1234: e~& = GetLastError~&: Print "last error:"; e~&
System
