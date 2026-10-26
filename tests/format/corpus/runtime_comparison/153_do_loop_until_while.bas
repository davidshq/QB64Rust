$Console:Only
' Runtime comparison: DO LOOP UNTIL vs DO LOOP WHILE
Dim n As Long
Print "DO LOOP UNTIL n>=2:"
n = 0
Do
    Print n;
    n = n + 1
Loop Until n >= 2
Print
Print "DO LOOP WHILE n<2:"
n = 0
Do
    Print n;
    n = n + 1
Loop While n < 2
Print
End
