' Slice (m2-console-screenhide, group 3): console INPUT and LINE INPUT in a program with a console and a hidden
' window. They read standard input while the console is the destination, with the prompts and the echo of
' $CONSOLE:ONLY; _SOURCE _CONSOLE is not needed and changes nothing. Standard input: s65_console_input.stdin.
$Console
$ScreenHide
_Dest _Console
Input "n"; a
Print "a ="; a
Line Input "l? "; l$
Print "l$ = ["; l$; "]"
Print "source:"; _Source
_Source _Console
Input "m"; b
Print "b ="; b
Line Input "k? "; k$
Print "k$ = ["; k$; "]"
Input "two, "; i%, t$
Print "i% ="; i%; "t$ = ["; t$; "]"
Input ; q&&
Print "q&& ="; q&&
readInSub
System

Sub readInSub
    Dim s As String
    Line Input ; s
    Print "s = ["; s; "]"
End Sub
