$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ only in a $IF branch not taken
$EMBED:'./v44_files/text.txt','h'
$IF 0 THEN
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
$END IF
PRINT "accepted"
SYSTEM
