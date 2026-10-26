$Console:Only
' The preprocessor (m2-parser-breadth task 7.1, design D8): predefined names on Windows 64-bit, $LET, $IF with
' $ELSEIF/$ELSE IF/$ELSE and both $END IF spellings, nested $IFs (a true one inside a skipped branch stays
' skipped), comparisons (=, <>, <, >=, VERSION), AND/OR/XOR, DEFINED/UNDEFINED, $LET of a predefined name not
' overriding it, a $IF inside a FOR body and a SUB, a whole SUB inside an active $IF, and skipped branches holding
' code that would not compile (an unclosed FOR, a SUB, garbage, an unknown metacommand).
$If WIN Then
    Print "win"
$Else
    PRINT "not win"
$End If
$If LINUX Then
    PRINT "linux"
$ElseIf MAC Then
    PRINT "mac"
$ElseIf 64BIT Then
    Print "64bit"
$Else
    PRINT "else"
$End If
$If 32BIT OR _ARM_ Then
    PRINT "32bit or arm"
    FOR i = 1 TO
$Else
    Print "neither 32bit nor arm"
$End If
$Let MODE = 2
$If MODE = 1 Then
    PRINT "mode 1"
$ElseIf MODE = 2 Then
    Print "mode 2"
$ElseIf MODE > = 2 Then
    PRINT "mode >= 2, not taken after a taken branch"
$End If
$Let MODE = 3
$If MODE > 2 AND WIN Then
    Print "mode > 2 and win"
$End If
$If MODE <> 3 XOR WIN Then
    Print "xor"
$End If
$If VERSION >= 4.0 Then
    Print "version >= 4.0"
$End If
$If VERSION < 3.9.1 Then
    PRINT "version < 3.9.1"
$End If
$If NOTDEFINED = UNDEFINED Then
    Print "undefined"
$End If
$If MODE = DEFINED Then
    Print "defined"
$End If
$Let WIN = 0
$If WIN Then
    Print "win still true"
$End If
$If LINUX Then
    $If WIN Then
        PRINT "nested, skipped"
    $Else
        this is not BASIC (
    $End If
    $FOO:BAR
    SUB never
    PRINT "never"
$Else
    $If 64BIT Then
        Print "nested, taken"
    $End If
$End If
For i = 1 To 2
    $If WIN Then
        Print "loop"; i
    $End If
Next
s
$If WIN Then
    t
$End If
System

Sub s
    $If MAC Then
        PRINT "mac sub"
    $Else
        Print "sub"
    $End If
End Sub

$If WIN Then
    Sub t
        Print "t"
    End Sub
$End If
