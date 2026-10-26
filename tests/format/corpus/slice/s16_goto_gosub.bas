$Console:Only
' GOTO, GOSUB, RETURN and labels (spec language/control-flow, "GOTO", "GOSUB and RETURN", "Labels per body"):
' jumps into a block (IF branch, FOR, WHILE, DO bodies) and out of one, a FOR entered by GOTO before it ever ran
' (its hidden limit and step are 0; capped), GOSUB nested and from inside a block, RETURN label, GOSUB inside a
' SUB, the same label name in main and in two SUBs, a label inside a block of a SUB, RETURN with nothing pending in
' main and in a SUB (error 3), GOSUB from a handler, RETURN in a SUB while a main GOSUB is pending (error 3, the
' entry is consumed).
On Error GoTo h
Print "GOTO into an IF branch whose condition is false"
x = 0
GoTo inif
If x = 1 Then
    Print "  then 1"
    inif:
    Print "  then 2"
Else
    Print "  else"
End If
Print "  after END IF"
Print "GOTO into a FOR that never ran (capped)"
n = 0
GoTo infor
For i = 1 To 3
    Print "  body 1"; i
    infor:
    n = n + 1: Print "  body 2"; i
    If n >= 3 Then Exit For
Next
Print "  after"; i
Print "GOTO out of a FOR, then the FOR again"
For i = 1 To 5
    If i = 2 Then GoTo outfor
    Print "  body"; i
Next
outfor:
Print "  out at"; i
For i = 1 To 2
    Print "  second run"; i
Next
Print "GOTO into a WHILE body"
n = 0
GoTo inwhile
While n < 3
    Print "  top"; n
    inwhile:
    n = n + 1: Print "  in"; n
    If n > 6 Then Exit While
Wend
Print "  after"; n
Print "GOTO into a DO ... LOOP UNTIL body"
n = 0
GoTo indo
Do
    Print "  top"; n
    indo:
    n = n + 1: Print "  in"; n
    If n > 6 Then Exit Do
Loop Until n >= 3
Print "  after"; n
Print "a backward GOTO as a loop"
n = 0
lab:
n = n + 1
If n < 3 Then GoTo lab
Print "  n"; n
Print "GOSUB"
GoSub g1
Print "  back from g1"
For i = 1 To 2
    GoSub g1
Next
Print "GOSUB nested"
GoSub outer
Print "  back from outer"
Print "RETURN label"
GoSub g2
Print "  not printed"
back2:
Print "  at back2"
GoSub g1
Print "  GOSUB after RETURN label works"
Print "SUBs with their own labels"
s1
s2
Print "RETURN with nothing pending"
Return
Print "  after RETURN"
sret
Print "  back from sret"
Print "GOSUB from a handler"
Error 9
Print "  after ERROR 9"
Print "RETURN in a SUB while a main GOSUB is pending (ends the program)"
GoSub g3
Print "  not printed: g3's RETURN has nothing to return to"
System

g1:
Print "  in g1"
Return

outer:
Print "  in outer"
GoSub inner
Print "  outer again"
Return

inner:
Print "  in inner"
Return

g2:
Print "  in g2"
Return back2

g3:
Print "  in g3"
sret
Print "  g3 after sret"
Return
Print "  after g3's RETURN"
System

hsub:
Print "  in hsub"
Return

h:
Print "  handler"; Err
If Err = 9 Then GoSub hsub
Resume Next

Sub s1
    Print "  in s1"
    GoSub lab
    Print "  s1 done"
    Exit Sub
    lab:
    Print "  lab in s1"
    Return
End Sub

Sub s2
    n = 0
    If n = 0 Then
        lab:
        n = n + 1
    End If
    If n < 3 Then GoTo lab
    Print "  s2 n"; n
    GoSub lab2
    Print "  s2 done"
    Exit Sub
    lab2:
    Print "  lab2 in s2"
    Return
End Sub

Sub sret
    Print "  in sret"
    Return
    Print "  sret after RETURN"
End Sub
