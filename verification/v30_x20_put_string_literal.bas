$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (20_put_string_literal)
ON ERROR GOTO h
OPEN "v30_x20_put_string_literal.tmp" FOR BINARY AS #1
PUT #1, , "lit"
CLOSE
KILL "v30_x20_put_string_literal.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
