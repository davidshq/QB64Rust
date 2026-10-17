$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ with two arguments
$EMBED:'./v44_files/text.txt','h'
PRINT LEN(_EMBEDDED$("h", 1))
SYSTEM
