$CONSOLE:ONLY
$EMBED:'./bytes.bin','bytes'
b$ = _EMBEDDED$("bytes")
PRINT "Bytes:"; LEN(b$); "first"; ASC(b$, 1); "last"; ASC(b$, LEN(b$))
MID$(b$, 1, 1) = "x"
PRINT "A new string at each call:"; ASC(_EMBEDDED$("bytes"), 1)
END
