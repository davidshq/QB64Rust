$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a relative name without ./
$EMBED:'v44_files/text.txt','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
