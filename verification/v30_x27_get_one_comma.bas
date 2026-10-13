$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (27_get_one_comma)
ON ERROR GOTO h
OPEN "v30_x27_get_one_comma.tmp" FOR BINARY AS #1
GET #1,
CLOSE
KILL "v30_x27_get_one_comma.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
