$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (26_get_two_commas_no_variable)
ON ERROR GOTO h
OPEN "v30_x26_get_two_commas_no_variable.tmp" FOR BINARY AS #1
GET #1, ,
CLOSE
KILL "v30_x26_get_two_commas_no_variable.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
