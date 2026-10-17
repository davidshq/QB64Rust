$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED with a space before the first quote and none between
$EMBED './v44_files/text.txt''h'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
