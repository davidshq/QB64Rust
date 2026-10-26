' Screen program (m4-qb64pe-builtins, task 2.1): _CONTROLCHR. With control characters on (the start), CHR$(12)
' clears the screen, and TAB, the cursor moves 28 to 31, CHR$(11) and CHR$(13) act; with them off, the characters
' 0 to 31 are glyphs on one row, and a bell and a line feed between two letters; the function before and after.
Print "gone"; Chr$(12);
Print _ControlChr
Locate 5, 1: Print "x"; Chr$(9); "y"
Locate 6, 1: Print "ab"; Chr$(28); "c"; Chr$(29); Chr$(29); "d"
Locate 8, 5: Print "e"; Chr$(30); "f"; Chr$(31); "g"
Locate 10, 10: Print "hh"; Chr$(13); "i"
Locate 12, 10: Print "j"; Chr$(11); "k"
_ControlChr Off
Locate 14, 1
For i = 0 To 31: Print Chr$(i);: Next
Print
Print "a" + Chr$(7) + Chr$(10) + "b"; _ControlChr
_ControlChr On
Print _ControlChr
System
