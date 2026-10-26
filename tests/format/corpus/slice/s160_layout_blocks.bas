$Console:Only
' Slice (m5b-format-settings, task 1.1): blocks of every kind laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list). The indentation is written wrong on purpose, the spacing
' is odd, the keywords are in mixed case, and each block line has a comment after it.
Type pair ' a record
    a As Integer ' first
    b As String * 3
End Type ' end of the record
Dim p As pair: p.a = 7: p.b = "xyz"
Print p.a; p.b
i% = 0
Do While i% < 3 ' a loop tested first
    i% = i% + 1
    If i% = 2 Then ' a block IF
        Print "two"
    ElseIf i% = 3 Then ' its ELSEIF
        Print "three"
    Else ' its ELSE
        Print "one"
    End If ' its END IF
Loop ' end of the loop
Do ' a loop tested last
    i% = i% - 1
Loop Until i% <= 0 ' its end
For j = 1 To 3 Step 1 ' a FOR
    For k = 1 To j ' a nested FOR
        Print j * k;
    Next k ' its NEXT
    Print
Next ' end of the outer FOR
While j > 1 ' a WHILE
    j = j - 1
Wend ' its WEND
For n = 1 To 5
    Select Case n ' a SELECT CASE
        Case 1 ' a CASE
            Print "a";
        Case 2, 3 ' a CASE with two values
            Print "b";
        Case Is > 4 ' CASE IS
            Print "c";
        Case Else ' CASE ELSE
            Print "d";
    End Select ' end of the SELECT
Next n
Print
If j = 1 Then Print "single line" Else Print "not" ' a single-line IF
If j = 1 Then
    Print "joined";: Print " in a block": i% = 0
End If
For q = 1 To 2: Print q;: Next: Print
10 Print "line ten" ' a line number
20 If j = 1 Then GoTo done ' a jump
Print "not reached"
done: ' a label
Print "done"
again: Print "label and statement"
