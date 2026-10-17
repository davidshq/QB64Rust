$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): the same file twice under two handles
$EMBED:'./v44_files/text.txt','h'
$EMBED:'./v44_files/text.txt','g'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
