$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (36_get_parenthesised_file_number)
ON ERROR GOTO h
OPEN "v30_x36_get_parenthesised_file_number.tmp" FOR BINARY AS #1
DIM c AS STRING * 1, k AS LONG
k = 1
GET (k), 1, c
PRINT SEEK(1)
CLOSE
KILL "v30_x36_get_parenthesised_file_number.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
