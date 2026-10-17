$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ in a CONST
$EMBED:'./v44_files/text.txt','h'
CONST c = _EMBEDDED$("h")
PRINT LEN(c)
SYSTEM
