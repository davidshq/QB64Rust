$CONSOLE:ONLY
$EMBED:'./greeting.txt','greeting'
g$ = _EMBEDDED$("greeting")
PRINT LEN(g$); "bytes: "; g$
END
