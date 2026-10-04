$CONSOLE:ONLY
' PRINT item forms: ';', trailing ';', auto-semicolon next to string literals, empty PRINT, mixed types.
' No ',' items: a comma zone hangs a $CONSOLE program whose output is redirected (study\10 section 2.2).
x = 7
y% = -2
s$ = "str"
PRINT "["; x; "]"
PRINT "[" "a"1; x "b" "]"
PRINT "[";
PRINT x;
PRINT "]"
PRINT
PRINT ;
PRINT "a";; "b"
PRINT x; y%; s$; 1.5; s$ + "!"; -x
PRINT s$; s$
PRINT "[" + s$ + "]"
PRINT -y%; - -y%; -(-y%)
LET z# = 1 / 4: PRINT z#
PRINT "end": PRINT "of": PRINT "items";
PRINT
END
