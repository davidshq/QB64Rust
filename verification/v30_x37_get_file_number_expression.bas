$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (37_get_file_number_expression)
ON ERROR GOTO h
OPEN "v30_x37_get_file_number_expression.tmp" FOR BINARY AS #1
DIM c AS STRING * 1, k AS LONG
k = 1
GET #k + 0, 1, c
PRINT SEEK(1)
CLOSE
KILL "v30_x37_get_file_number_expression.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
