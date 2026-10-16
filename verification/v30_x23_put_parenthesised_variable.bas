$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (23_put_parenthesised_variable)
ON ERROR GOTO h
OPEN "v30_x23_put_parenthesised_variable.tmp" FOR BINARY AS #1
DIM i AS INTEGER
PUT #1, , (i)
CLOSE
KILL "v30_x23_put_parenthesised_variable.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
