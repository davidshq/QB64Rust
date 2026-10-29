$Console:Only
' _ERRORLINE, _INCLERRORLINE, _INCLERRORFILE$ and _ERRORMESSAGE$ in the main file (m3-errorline-chr-const;
' verification\v43_a_errorline, v43_d_continued, v43_x44): before any error, after errors on several lines, after
' RESUME NEXT, in a SUB, a FUNCTION and a GOSUB, a line with several statements, blank lines and comments, lines
' continued with an underscore (a line joined to the next is numbered as the last line of the join), a line in a
' $IF branch, the message of codes given as an argument, rounded half to even.
On Error GoTo handler
Print "1. before any error:"; _ErrorLine; _InclErrorLine; "["; _InclErrorFile$; "] ["; _ErrorMessage$; "]"
Error 5
Print "2. after RESUME NEXT:"; _ErrorLine

Print "3."
Error 6
x = 1: Error 7: y = 2
' a comment

a = 1: b = 0: Error 8
s
z = f(3)
Print "4. back in main:"; _ErrorLine; Err
Print "5. messages: ["; _ErrorMessage$(5); "] ["; _ErrorMessage$(0); "] ["; _ErrorMessage$(1000); "] ["; _ErrorMessage$(-1); "]"
Dim e As Long: e = 12
Print "6. ["; _ErrorMessage$(e); "] ["; _ErrorMessage$(9.6); "] ["; _ErrorMessage$(52.5); "]"
Error 255
Print "7. continued:"
ERROR _
12
$IF WIN THEN
    Error 13
$End If
Print "8. arithmetic:"; _ErrorLine + 0.5; _ErrorLine \ 2
ERROR 14: x = 1 + _
2
x = 1 + _
2: ERROR 15
ERROR _
_
16
IF x = 3 THEN _
ERROR 17
IF x = _
3 THEN
    Error 18
End If
GoSub g
Print "9. after the GOSUB:"; _ErrorLine
System

g:
Error 19
Return

handler:
Print "  error"; Err; "_ERRORLINE"; _ErrorLine; "_INCLERRORLINE"; _InclErrorLine; "file ["; _InclErrorFile$; "] message ["; _ErrorMessage$; "]"
Resume Next

Sub s
Print "  in s"
Error 9
End Sub

Function f (n)
Error 10
f = n
End Function
