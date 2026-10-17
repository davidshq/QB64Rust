$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a handle of 40 letters and digits
$EMBED:'./v44_files/text.txt','a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1'
PRINT LEN(_EMBEDDED$("a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1"))
SYSTEM
