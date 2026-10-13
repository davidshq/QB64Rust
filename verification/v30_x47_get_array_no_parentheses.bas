$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (47_get_array_no_parentheses)
ON ERROR GOTO h
OPEN "v30_x47_get_array_no_parentheses.tmp" FOR BINARY AS #1
DIM a(1 TO 3) AS INTEGER
GET #1, 1, a
PRINT SEEK(1)
CLOSE
KILL "v30_x47_get_array_no_parentheses.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
