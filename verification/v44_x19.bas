$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED without the $
$EMBED:'./v44_files/text.txt','h'
PRINT LEN(_EMBEDDED("h"))
SYSTEM
