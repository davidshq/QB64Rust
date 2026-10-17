$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED in a comment
'$EMBED:'./v44_files/text.txt','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
