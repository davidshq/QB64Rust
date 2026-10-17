$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED inside a SUB, used in the main module
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
SUB s
$EMBED:'./v44_files/text.txt','h'
END SUB
