$Console:Only
' Slice (m5b-format-settings, task 1.1): the text forms of the format oracle under the other layout settings
' (tests\format\settings.list): an unclosed string at the end of a line, '$FORMAT:OFF and '$FORMAT:ON, ?, joined
' lines, REM, DATA, and spacing written by hand.
Print "question mark"
Print "no space";1;2
a=1:b =2:   c= a+b  :Print a;b;c
Print "unclosed at the end"
x$ = "also unclosed:   kept as written"
Print x$
'$Format:Off
    If a=1   Then
  Print   "format off: as written"
        End If
'$Format:On
If a = 1 Then
    Print "format on again"
End If
Rem    a REM line
Print a   :   Rem a REM after a statement
Read d1,d2$
Print d1;d2$
Data   12 ,  hello there  ,"x"
Print  Using   "##.##";3.14159
Print   Chr$(65)+  LCase$("BC")  ;  Len( "abc" )
s$=String$(3,"*") : Print s$
If Not (a = 2 And b = 2) Or c = 3 Then Print "logic"
y# = -a ^ 2 + b Mod 2 - c \ 2: Print y#
