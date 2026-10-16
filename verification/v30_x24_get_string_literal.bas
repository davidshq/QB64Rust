$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (24_get_string_literal)
ON ERROR GOTO h
OPEN "v30_x24_get_string_literal.tmp" FOR BINARY AS #1
GET #1, , "lit"
CLOSE
KILL "v30_x24_get_string_literal.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
