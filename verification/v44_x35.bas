$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED with a comment after it
$EMBED:'./v44_files/text.txt','h' ' a note
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
