$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ of a handle no $EMBED names
$EMBED:'./v44_files/text.txt','h'
PRINT LEN(_EMBEDDED$("nope"))
SYSTEM
