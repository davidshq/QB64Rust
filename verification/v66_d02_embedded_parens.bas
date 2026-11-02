$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-network-embed row): _EMBEDDED$ with its argument in parentheses (the program embeds itself).
$EMBED:'./v66_d02_embedded_parens.bas','h'
PRINT LEFT$(_EMBEDDED$(("h")), 13)
SYSTEM
