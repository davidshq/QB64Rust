$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): blanks inside the literal of _EMBEDDED$
$EMBED:'./v44_files/text.txt','h'
PRINT LEN(_EMBEDDED$(" h "))
SYSTEM
