$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _EMBEDDED$ of a variable
$EMBED:'./v44_files/text.txt','h'
h$ = "h"
PRINT LEN(_EMBEDDED$(h$))
SYSTEM
