$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ without an argument
$EMBED:'./v44_files/text.txt','h'
PRINT LEN(_EMBEDDED$)
SYSTEM
