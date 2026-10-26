$Console:Only
' IF (spec language/control-flow, "IF"): block IF with ELSEIF chains and ELSE, nesting, conditions as numbers
' (true when not zero), single-line IF with and without ELSE (the statements after ELSE belong to the ELSE part),
' a single-line IF inside a single-line IF, IF ... GOTO and IF ... THEN GOTO.
Print "ELSEIF chain"
pick 1
pick 2
pick 3
pick 7
Print "conditions are numbers"
If 5 Then Print "  5 is true"
If -1 Then Print "  -1 is true"
If 0.4 Then Print "  0.4 is true" Else Print "  0.4 is false"
If 0 Then Print "  0 is true" Else Print "  0 is false"
If 2 And 1 Then Print "  2 AND 1 is true" Else Print "  2 AND 1 is false"
c$ = "b"
If c$ = "b" Then Print "  c$ is b"
If c$ <> "b" Then Print "  c$ is not b" Else Print "  c$ is still b"
Print "single-line IF with ELSE"
If 0 Then Print "a" Else Print "b": Print "c"
If 1 Then Print "a": Print "a2" Else Print "b": Print "c"
Print "d"
Print "single-line IF without ELSE"
If 0 Then Print "never": Print "never 2"
Print "  after"
Print "single-line IF inside a single-line IF"
If 1 Then If 0 Then Print "  inner then" Else Print "  inner else"
If 0 Then If 1 Then Print "  inner then" Else Print "  which else?"
Print "  after"
Print "nested block IF"
a = 1: b = 0
If a Then
    If b Then
        Print "  a and b"
    Else
        Print "  a, not b"
    End If
    Print "  still in a"
ElseIf b Then
    Print "  b only"
End If
If b Then
    Print "  b"
ElseIf a Then
    If a = 1 Then Print "  a is 1" Else Print "  a is not 1"
End If
Print "block IF, false, no ELSE"
If a = 2 Then
    Print "  never"
End If
Print "  after END IF"
Print "IF ... GOTO and IF ... THEN GOTO"
n = 0
again:
n = n + 1
Print "  pass"; n
If n < 3 GoTo again
If n = 3 Then GoTo jumped
Print "  not printed"
jumped:
Print "  after the jump"
If n = 4 Then GoTo again Else Print "  no jump"
System

Sub pick (x)
    If x = 1 Then
        Print "  one"
    ElseIf x = 2 Then
        Print "  two"
    ElseIf x > 2 And x < 5 Then
        Print "  three or four"
    Else
        Print "  other"; x
    End If
End Sub
