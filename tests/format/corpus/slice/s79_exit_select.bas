$Console:Only
' Slice (m3-console-files, task 3.1; from verification\v30_f_exit_select, v30_x62, x65, x66): EXIT SELECT and EXIT
' CASE, 12 numbered lines. EXIT SELECT from a CASE, from an IF and from loops inside it, from CASE ELSE; a loop
' around the SELECT goes on; the inner of two SELECTs; SELECT EVERYCASE, where EXIT CASE goes on with the next CASE
' and the block still counts as matched; a single-line IF; in a SUB; a string selector; and EXIT CASE before a
' SELECT in the same CASE, which goes on after that inner SELECT (the old compiler's count of CASEs).
Dim n As Long, k As Long, v As String
Print "1:";
For n = 1 To 4
    Select Case n
        Case 1
            Print " one";
            Exit Select
            Print " not reached";
        Case 2
            Print " two";
            If n = 2 Then Exit Select
            Print " not reached";
        Case 3
            Print " three";
            If n = 3 Then
                Print " in IF";
                Exit Select
            End If
            Print " not reached";
        Case Else
            Print " else";
            Exit Select
            Print " not reached";
    End Select
    Print n;
Next
Print
Print "2:";
Select Case 1
    Case 1
        For k = 1 To 3
            Print k;
            If k = 2 Then Exit Select
        Next
        Print "not reached";
End Select
Print "after"; k
k = 0
Select Case 1
    Case 1
        Do
            k = k + 1
            While k < 5
                k = k + 1
                If k = 3 Then Exit Select
            Wend
        Loop
End Select
Print "3: after"; k
Print "4:";
Select Case 1
    Case 1
        Select Case 2
            Case 2
                Print " inner";
                Exit Select
                Print " not reached";
        End Select
        Print " outer goes on";
        Exit Select
        Print " not reached";
End Select
Print " after"
Print "5:";
For k = 1 To 4
    Select Case k
        Case 2
            Exit Select
        Case 4
            Exit For
        Case Else
            Print k;
    End Select
Next
Print "after"; k
Print "6:";
Select EveryCase 5
    Case 5
        Print " first";
        Exit Select
        Print " not reached";
    Case Is > 1
        Print " second";
    Case Else
        Print " else";
End Select
Print " after"
Print "7:";
Select EveryCase 5
    Case 5
        Print " first";
        Exit Case
        Print " not reached";
    Case Is > 1
        Print " second";
        Exit Case
        Print " not reached";
    Case 7
        Print " third";
    Case Else
        Print " else";
End Select
Print " after"
Print "8:";
Select EveryCase 5
    Case 1
        Print " first";
    Case Else
        Print " else";
        Exit Case
        Print " not reached";
End Select
Select Case 1
    Case 1
        Print " plain";
        Exit Case
        Print " not reached";
    Case Else
        Print " else";
End Select
Print " after"
Print "9:";
n = 0
Select Case 1
    Case 1
        If n Then Print " a"; Else Exit Select
        Print " not reached";
End Select
t 1: t 2: t 3
Print " after"
v = "b"
Print "10:";
Select Case v + "c"
    Case "bc"
        Print " string";
        If Len(v) = 1 Then Exit Select
        Print " not reached";
    Case Else
        Print " else";
End Select
Print " after"
Print "11:";
Select Case 1
    Case 1
        Print " a";
        If n = 0 Then Exit Case
        Print " skipped";
        Select Case 2
            Case 2
                Print " inner";
            Case 3
                Print " inner 3";
        End Select
        Print " after the inner SELECT";
    Case Else
        Print " else";
End Select
Print " end"
Print "12:";
Select EveryCase 1
    Case 1
        Print " b";
        If n = 0 Then Exit Case
        Print " skipped";
        Select EveryCase 2
            Case 2
                Print " inner";
            Case Is > 1
                Print " inner second";
            Case Else
                Print " inner else";
        End Select
        Print " after the inner SELECT";
    Case Is > 0
        Print " second";
    Case Else
        Print " else";
End Select
Print " end"
System
Sub t (n As Long)
    Select Case n
        Case 1: Print " one";: Exit Select: Print " not reached";
        Case 2: Print " two";: Exit Sub
        Case Else: Exit Case
    End Select
    Print " end of t"; n;
End Sub
