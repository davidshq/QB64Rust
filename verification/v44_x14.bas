$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ before its $EMBED line
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
$EMBED:'./v44_files/text.txt','h'
SYSTEM
