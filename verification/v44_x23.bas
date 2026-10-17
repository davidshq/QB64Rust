$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $embed in lower case
$embed:'./v44_files/text.txt','h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
