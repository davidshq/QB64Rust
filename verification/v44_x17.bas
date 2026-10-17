$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED in a $IF branch not taken
$IF 0 THEN
$EMBED:'./v44_files/text.txt','h'
$END IF
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
