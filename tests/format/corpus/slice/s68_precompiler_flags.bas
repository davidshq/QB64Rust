' Slice (m2-console-screenhide, group 6): the precompiler flags in $IF, with the values the old compiler gives
' them from the whole program. This program needs three rounds: its only OPTION _EXPLICIT stands inside the
' branch its own flag switches off (met in the first round, in force by the flag from then on), and its only
' $CONSOLE inside $IF _EXPLICIT_ (met in the second round). Every line is then read with _CONSOLE_ 1 and
' _EXPLICIT_ 1, also the lines above the ones that set them. The two programs beside it (suffixes b and c) hold
' the settings one program cannot hold together.
$ScreenHide
_Dest _Console
$If _CONSOLE_ = 1 Then
    Print "above the line of $CONSOLE: 1"
$ElseIf _CONSOLE_ = 2 Then
    PRINT "above the line of $CONSOLE: 2"
$Else
    PRINT "above the line of $CONSOLE: 0"
$End If
$If _EXPLICIT_ Then
    Print "above the option: explicit 1"
    $Console
$Else
    PRINT "above the option: explicit 0"
$End If
$If _EXPLICIT_ = 0 Then
    OPTION _EXPLICIT
    PRINT "the branch of the option is compiled"
$End If
$If _EXPLICITARRAY_ Then
    Print "explicitarray 1"
$Else
    PRINT "explicitarray 0"
$End If
$If _ASSERTS_ OR _DEBUG_ OR _SOCKETS_ Then
    PRINT "another flag is set"
$Else
    Print "asserts, debug, sockets 0"
$End If
$If _CONSOLE_ = 1 AND _EXPLICIT_ = 1 Then
    Print "console 1 and explicit 1"
$End If
$If _CONSOLE_ <> 0 AND WINDOWS Then
    Print "a flag beside a predefined name"
$End If
$If _CONSOLE_ = 0 Then
    PRINT "not printed"
    $If _EXPLICIT_ Then
        PRINT "not printed either"
    $End If
$ElseIf _CONSOLE_ > = 1 Then
    Print "console >= 1"
$End If
Dim declared As Long
declared = _Console
Print "the console's handle:"; declared
show
System

Sub show
    $If _CONSOLE_ Then
        Print "in a SUB: console"
    $Else
        PRINT "in a SUB: no console"
    $End If
End Sub
