$Console:Only
' Errors in block headers (spec language/error-handling, "Errors in block headers"; spec compiler/pipeline, the
' pending-error rule): each header kind with RESUME NEXT, then with RESUME, then untrapped (run with
' QB64PE_NOPROMPT=continue, s19_header_errors.noprompt). CHR$(-1) raises error 5 and gives "" as its placeholder.
' Also a store made with a placeholder value, a SUB called with a raising argument, and an ELSEIF that raises
' (tested with the placeholder; when false, the error is serviced at the next statement that runs).
' Caps: the RESUME NEXT handler makes the call succeed every third error of a case (k = 65), every handler stops
' the program after 30 errors of a case, the loops of the RESUME NEXT and untrapped parts leave after a few passes,
' and those of the RESUME part are bounded by their counters. A FOR case that should show its stored limit or
' step starts with the variable at 0, since the body sees its old value.
Dim k As Long
On Error GoTo hnext
Print "RESUME NEXT"
Print "single-line IF"
k = -1: errs = 0
If Chr$(k) = "a" Then Print "  then" Else Print "  else"
Print "block IF"
k = -1: errs = 0
If Chr$(k) = "a" Then
    Print "  then"
Else
    Print "  else"
End If
Print "ELSEIF, the placeholder makes it false"
k = -1: errs = 0: x = 1
If x = 2 Then
    Print "  if"
ElseIf Chr$(k) = "a" Then
    Print "  elseif"
Else
    Print "  else"
    Print "  else 2"
End If
Print "ELSEIF, the placeholder makes it true"
k = -1: errs = 0
If x = 2 Then
    Print "  if"
ElseIf Chr$(k) = "" Then
    Print "  elseif"
Else
    Print "  else"
End If
Print "two ELSEIFs, the first raises and is false"
k = -1: errs = 0
If x = 2 Then
    Print "  if"
ElseIf Chr$(k) = "a" Then
    Print "  elseif 1"
ElseIf x = 1 Then
    Print "  elseif 2"
Else
    Print "  else"
End If
Print "IF ... GOTO"
k = -1: errs = 0
If Chr$(k) = "a" GoTo jumped
Print "  not jumped"
jumped:
Print "  after"
Print "WHILE"
k = -1: errs = 0: n = 0
While Chr$(k) <> "" And n < 2
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit While
Wend
Print "DO WHILE"
k = -1: errs = 0: n = 0
Do While Chr$(k) <> "" And n < 2
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit Do
Loop
Print "DO UNTIL"
k = -1: errs = 0: n = 0
Do Until Chr$(k) = "" Or n >= 2
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit Do
Loop
Print "LOOP WHILE"
k = -1: errs = 0: n = 0
Do
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit Do
Loop While Chr$(k) <> "" And n < 3
Print "  after"; n
Print "LOOP UNTIL"
k = -1: errs = 0: n = 0
Do
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit Do
Loop Until Chr$(k) = "a" Or n >= 3
Print "  after"; n
Print "FOR limit"
k = -1: errs = 0: n = 0
For i = 1 To InStr("xyz", Chr$(k)) + 2
    n = n + 1: Print "  body"; i
    If n > 6 Then Exit For
Next
Print "  after"; i
Print "FOR start"
k = -1: errs = 0: n = 0: i = 7
For i = InStr("xyA", Chr$(k)) To 4
    n = n + 1: Print "  body"; i
    If n > 6 Then Exit For
Next
Print "  after"; i
Print "FOR step"
k = -1: errs = 0: n = 0: i = 0
For i = 1 To 3 Step InStr("xA", Chr$(k))
    n = n + 1: Print "  body"; i
    If n > 6 Then Exit For
Next
Print "  after"; i
Print "a store made with a placeholder value"
k = -1: errs = 0
s$ = "a": s$ = Chr$(k)
Print "  ["; s$; "]"
s$ = "a": s$ = "<" + Chr$(k) + ">"
Print "  ["; s$; "]"
y = 5: y = InStr("abc", Chr$(k))
Print "  y"; y
Print "a SUB called with a raising argument"
k = -1: errs = 0
showstr Chr$(k)
Print "  after showstr"

On Error GoTo hretry
Print "RESUME"
Print "IF"
k = -1: errs = 0
If Chr$(k) = "A" Then Print "  then" Else Print "  else"
Print "ELSEIF"
k = -1: errs = 0
If k = 5 Then
    Print "  if"
ElseIf Chr$(k) = "A" Then
    Print "  elseif"
Else
    Print "  else"
End If
Print "WHILE"
k = -1: errs = 0: n = 0
While Chr$(k) = "A" And n < 2
    n = n + 1: Print "  body"; n
Wend
Print "DO WHILE"
k = -1: errs = 0: n = 0
Do While Chr$(k) = "A" And n < 2
    n = n + 1: Print "  body"; n
Loop
Print "DO UNTIL"
k = -1: errs = 0: n = 0
Do Until Chr$(k) <> "A" Or n >= 2
    n = n + 1: Print "  body"; n
Loop
Print "LOOP WHILE"
k = -1: errs = 0: n = 0
Do
    n = n + 1: Print "  body"; n
Loop While Chr$(k) = "A" And n < 3
Print "LOOP UNTIL"
k = -1: errs = 0: n = 0
Do
    n = n + 1: Print "  body"; n
Loop Until Chr$(k) <> "A" Or n >= 3
Print "FOR start"
k = -1: errs = 0
For i = InStr("xyA", Chr$(k)) To 4
    Print "  body"; i
Next
Print "  after"; i
Print "FOR limit"
k = -1: errs = 0
For i = 1 To InStr("xyA", Chr$(k))
    Print "  body"; i
Next
Print "  after"; i
Print "FOR step"
k = -1: errs = 0
For i = 1 To 5 Step InStr("xA", Chr$(k))
    Print "  body"; i
Next
Print "  after"; i

On Error GoTo 0
Print "untrapped"
Print "IF"
k = -1
If Chr$(k) = "A" Then Print "  then" Else Print "  else"
Print "WHILE"
n = 0
While Chr$(k) = "A" And n < 2
    n = n + 1: Print "  body"; n
    If n > 2 Then Exit While
Wend
Print "LOOP UNTIL"
n = 0
Do
    n = n + 1: Print "  body"; n
    If n > 6 Then Exit Do
Loop Until Chr$(k) = "A" Or n >= 3
Print "FOR limit"
n = 0: i = 0
For i = 1 To InStr("xyz", Chr$(k)) + 2
    n = n + 1: Print "  body"; i
    If n > 6 Then Exit For
Next
Print "  after"; i
Print "end"
System

hnext:
errs = errs + 1
Print "  handler"; Err
If errs Mod 3 = 0 Then k = 65
If errs > 30 Then Print "too many errors": System
Resume Next

hretry:
errs = errs + 1
Print "  handler"; Err
If errs > 30 Then Print "too many errors": System
k = 65
Resume

Sub showstr (t As String)
    Print "  in showstr ["; t; "]"
End Sub
