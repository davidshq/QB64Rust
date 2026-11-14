$CONSOLE:ONLY
c$ = _DEFLATE$("hello world")
PRINT "["; _INFLATE$(c$); "]"
PRINT "The first 3 bytes: ["; _INFLATE$(c$, 3); "]"
END
