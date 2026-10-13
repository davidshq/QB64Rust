$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (07_using_file_comma_after_format)
ON ERROR GOTO h
OPEN "v30_x07_using_file_comma_after_format.tmp" FOR BINARY AS #1
PRINT #1, USING "##", 5
CLOSE
KILL "v30_x07_using_file_comma_after_format.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
