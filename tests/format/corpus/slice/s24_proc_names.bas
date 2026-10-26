$Console:Only
' Procedures named like built-ins (verification\v19_proc_names.txt): a SUB may take a built-in function's name
' (LOC, ABS, FRE), a FUNCTION a built-in statement's name (BEEP, WIDTH, CLOSE), both the bare name of a built-in
' written with `$` (LEFT, CHR). Called bare, with CALL and with arguments.
loc
Call loc
abs -2.5
fre 1, "x"
Print beep; width&; close + 1
left
Print chr(65)
System

Sub loc
    Print "loc"
End Sub

Sub abs (x)
    Print "abs"; x
End Sub

Sub fre (n As Long, s As String)
    Print "fre"; n; s
End Sub

Function beep
    beep = 1
End Function

Function width&
    width& = 2
End Function

Function close
    close = 3
End Function

Sub left
    Print "left"
End Sub

Function chr (n As Long)
    chr = n + 1
End Function
