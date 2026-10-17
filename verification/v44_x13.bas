$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): handles differ in letter case
$EMBED:'./v44_files/text.txt','Abc'
PRINT LEN(_EMBEDDED$("abc"))
SYSTEM
