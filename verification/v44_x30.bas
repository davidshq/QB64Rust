$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): two handles that differ in letter case
$EMBED:'./v44_files/text.txt','h'
$EMBED:'./v44_files/odd.bin','H'
PRINT LEN(_EMBEDDED$("h")); LEN(_EMBEDDED$("H"))
SYSTEM
