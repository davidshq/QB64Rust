$Console:Only
' The error functions for an error in an included file (m3-errorline-chr-const; verification\v43_b_include): in
' a file, in a file it includes, in a SUB of an included file called from main, and in main after each include.
' _ERRORLINE there is the main file's line of the outermost $INCLUDE;
' _INCLERRORLINE and _INCLERRORFILE$ the line in the innermost file and its name. The included files are in this
' program's .files folder.
On Error GoTo handler
Print "1."
'$Include:'inc/raise1.bi'
Error 5
Print "2."
'$Include:'inc/outer.bi'
Error 6
Print "3."
insub
Error 7
Print "4."
System

handler:
Print "  error"; Err; "_ERRORLINE"; _ErrorLine; "_INCLERRORLINE"; _InclErrorLine; "file ["; _InclErrorFile$; "]"
Resume Next

'$Include:'inc/subs.bi'
