$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (21_put_number_literal)
ON ERROR GOTO h
OPEN "v30_x21_put_number_literal.tmp" FOR BINARY AS #1
PUT #1, , 7
CLOSE
KILL "v30_x21_put_number_literal.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
