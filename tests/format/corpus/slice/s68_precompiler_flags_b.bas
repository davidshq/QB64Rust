$Console:Only
$Console
' Slice (m2-console-screenhide, group 6): the precompiler flags, second program. $CONSOLE:ONLY and below it
' $CONSOLE: _CONSOLE_ is 1, the value asked for last, and the program has no window. A setting inside a branch
' that a line above has switched off is met in no round: the OPTION _EXPLICIT inside $IF _CONSOLE_ = 0 stands
' below the lines that set the flag, so _EXPLICIT_ stays 0 and a variable needs no declaration. OPTION
' _EXPLICITARRAY on the last line sets _EXPLICITARRAY_ alone.
$If _CONSOLE_ = 0 Then
    OPTION _EXPLICIT
    PRINT "not printed"
$End If
$If _CONSOLE_ = 1 Then
    Print "console 1"
$ElseIf _CONSOLE_ = 2 Then
    PRINT "console 2"
$Else
    PRINT "console 0"
$End If
$If _EXPLICIT_ Then
    PRINT "explicit 1"
$Else
    Print "explicit 0"
$End If
$If _EXPLICITARRAY_ Then
    Print "explicitarray 1"
$Else
    PRINT "explicitarray 0"
$End If
notDeclared = 5
Print "a variable that is not declared:"; notDeclared
Dim a(3) As Long
a(1) = 7
Print "a declared array:"; a(1)
Print "destination:"; _Dest; _Console
System
Option _ExplicitArray
