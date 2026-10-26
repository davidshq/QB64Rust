$Console:Only
' DO and WHILE (spec language/control-flow, "DO and WHILE loops", "EXIT from loops"): WHILE, DO WHILE and DO UNTIL
' test before each pass, LOOP WHILE and LOOP UNTIL after it; DO ... LOOP left by EXIT DO; EXIT DO and EXIT WHILE
' from inside other blocks; EXIT leaves only the innermost block of its kind; loops in a SUB.
Print "WHILE"
n = 0
While n < 3
    n = n + 1: Print n;
Wend
Print
While n < 0
    Print "  never"
Wend
Print "DO WHILE"
n = 0
Do While n < 3
    n = n + 1: Print n;
Loop
Print
Print "DO UNTIL"
n = 0
Do Until n >= 3
    n = n + 1: Print n;
Loop
Print
Do Until n > 0
    Print "  never"
Loop
Print "LOOP WHILE"
n = 5
Do
    Print n;: n = n + 1
Loop While n < 3
Print
n = 0
Do
    n = n + 1: Print n;
Loop While n < 3
Print
Print "LOOP UNTIL"
n = 5: Do: Print n;: n = n + 1: Loop Until n > 3: Print
n = 0
Do
    n = n + 1: Print n;
Loop Until n = 3
Print
Print "DO ... LOOP with EXIT DO"
n = 0
Do
    n = n + 1
    If n = 4 Then Exit Do
    Print n;
Loop
Print
Print "  after"; n
Print "EXIT WHILE from inside a block IF"
w = 0
While 1
    w = w + 1
    If w = 4 Then
        Print "  leaving at"; w
        Exit While
    End If
Wend
Print "  after"; w
Print "EXIT DO leaves the inner DO only"
o = 0
Do While o < 2
    o = o + 1
    i = 0
    Do
        i = i + 1
        If i = 3 Then Exit Do
    Loop
    Print "  outer"; o; "inner"; i
Loop
Print "EXIT DO from inside a WHILE inside a DO"
o = 0
Do
    o = o + 1
    w = 0
    While w < 5
        w = w + 1
        If o = 2 And w = 3 Then Exit Do
    Wend
    Print "  pass"; o; w
Loop
Print "  after"; o; w
Print "string condition through a comparison"
s$ = ""
Do While s$ <> "aaa"
    s$ = s$ + "a"
Loop
Print "  "; s$
While s$ < "aaaaa"
    s$ = s$ + "a"
Wend
Print "  "; s$
Print "loops in a SUB"
countdown 3
countdown 0
System

Sub countdown (m)
    Print " ";
    Do While m > 0
        Print m;
        m = m - 1
    Loop
    Do
        Print "done";
    Loop Until 1
    Print
End Sub
