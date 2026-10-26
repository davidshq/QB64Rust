$Console:Only
' PRINT item forms: ';', trailing ';', auto-semicolon next to string literals, empty PRINT, mixed types.
' No ',' items: a comma zone hangs a $CONSOLE program whose output is redirected (study\10 section 2.2).
x = 7
y% = -2
s$ = "str"
Print "["; x; "]"
Print "["; "a"; 1; x; "b"; "]"
Print "[";
Print x;
Print "]"
Print
Print ;
Print "a"; "b"
Print x; y%; s$; 1.5; s$ + "!"; -x
Print s$; s$
Print "[" + s$ + "]"
Print -y%; --y%; -(-y%)
Let z# = 1 / 4: Print z#
Print "end": Print "of": Print "items";
Print
End
