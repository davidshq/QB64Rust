$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): one handle for two files
$EMBED:'./v44_files/text.txt','h'
$EMBED:'./v44_files/odd.bin','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
