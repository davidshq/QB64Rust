$Console:Only
' Slice (m3-window-output, task 3.3): the screen and keyboard functions in a program with a console and no window.
' With standard input and output redirected INKEY$ is empty, POS(0) is 1 wherever the output stands and CSRLIN
' does not move as lines are printed (verification\v27_a_console_functions), so only values that are the same on
' every run are printed: never CSRLIN itself.
Dim k As String, r As Long
k = InKey$
Print "inkey:"; Len(k); Len(InKey$ + "x")
r = CsrLin
Print "pos:"; Pos(0); Pos(1) * 2; " "; Hex$(Pos(0))
Print "abc"; Pos(0)
Print "csrlin:"; CsrLin - r; CsrLin = r
System
