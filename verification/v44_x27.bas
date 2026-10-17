$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a folder as the file
$EMBED:'./v44_files','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
