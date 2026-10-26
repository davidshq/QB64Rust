$Console:Only
' Mix: WHILE, DO LOOP, EXIT
n = 0
While n < 2
    n = n + 1
    Print "w"; n;
Wend
Print
Do
    n = n - 1
    Print "d"; n;
    If n <= 0 Then Exit Do
Loop
Print
End
