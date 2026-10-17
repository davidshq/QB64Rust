$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): blanks around the colon
$EMBED : './v44_files/text.txt','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
