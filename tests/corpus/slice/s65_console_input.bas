' Slice (m2-console-screenhide, group 3): console INPUT and LINE INPUT in a program with a console and a hidden
' window. They read standard input while the console is the destination, with the prompts and the echo of
' $CONSOLE:ONLY; _SOURCE _CONSOLE is not needed and changes nothing. Standard input: s65_console_input.stdin.
$CONSOLE
$SCREENHIDE
_DEST _CONSOLE
INPUT "n"; a
PRINT "a ="; a
LINE INPUT "l? "; l$
PRINT "l$ = ["; l$; "]"
PRINT "source:"; _SOURCE
_SOURCE _CONSOLE
INPUT "m"; b
PRINT "b ="; b
LINE INPUT "k? "; k$
PRINT "k$ = ["; k$; "]"
INPUT "two, "; i%, t$
PRINT "i% ="; i%; "t$ = ["; t$; "]"
INPUT ; q&&
PRINT "q&& ="; q&&
readInSub
SYSTEM

SUB readInSub
    DIM s AS STRING
    LINE INPUT ; s
    PRINT "s = ["; s; "]"
END SUB
