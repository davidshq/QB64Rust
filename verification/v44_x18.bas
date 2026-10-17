$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ of a sum of literals
$EMBED:'./v44_files/text.txt','ab'
PRINT LEN(_EMBEDDED$("a" + "b"))
SYSTEM
