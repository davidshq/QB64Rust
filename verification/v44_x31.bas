$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): the same $EMBED line twice
$EMBED:'./v44_files/text.txt','h'
$EMBED:'./v44_files/text.txt','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
