' CLS with its arguments and VIEW PRINT in SCREEN 0: a text viewport scrolls and is cleared alone.
Dim i As Integer
Color 15, 4
Cls
Print "white on red after CLS"
Cls , 2
Print "after CLS with a background"
For i = 3 To 20
    Locate i, 1
    Print "row"; i;
Next i
View Print 5 To 10
Print "in the viewport"
For i = 1 To 9
    Print "scrolls"; i
Next i
View Print 12 To 14
Cls 2
Print "cleared by CLS 2"
View Print 16 To 18
Cls
Print "cleared by CLS"
View Print
Locate 22, 1
Print "the whole screen again";
Color 7, 0
System
