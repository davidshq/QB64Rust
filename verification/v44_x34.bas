$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED after the last SUB
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
SUB s
END SUB
$EMBED:'./v44_files/text.txt','h'
