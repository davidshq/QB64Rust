$Console:Only
' _CONTINUE (m4-devices-continue-paths, design D1; verification\v47_a_continue): the next pass of the innermost
' FOR, DO or WHILE loop, from a single-line IF and from a SELECT CASE (as upstream colors/color_with_dest), with the
' step of NEXT and the test of LOOP UNTIL still run; in a SUB.
For i = 1 To 7
    Select Case i
        Case 3, 6: _Continue
    End Select
    Print i;
Next
Print
n = 0
Do
    n = n + 1
    If n < 3 Then _Continue
    Print n;
Loop Until n >= 5
Print
n = 0
While n < 4
    n = n + 1
    For j = 1 To 3
        If j = 2 Then _Continue
        Print n * 10 + j;
    Next
    If n = 2 Then _Continue
    Print "/";
Wend
Print
odd 6
System

Sub odd (m)
    For k = 1 To m
        If k Mod 2 = 0 Then _Continue
        Print k;
    Next
    Print
End Sub
