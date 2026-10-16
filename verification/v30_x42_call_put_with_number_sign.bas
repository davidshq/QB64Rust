$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (42_call_put_with_number_sign)
ON ERROR GOTO h
OPEN "v30_x42_call_put_with_number_sign.tmp" FOR BINARY AS #1
DIM c AS STRING * 1
CALL PUT(#1, 1, c)
PRINT SEEK(1)
CLOSE
KILL "v30_x42_call_put_with_number_sign.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
