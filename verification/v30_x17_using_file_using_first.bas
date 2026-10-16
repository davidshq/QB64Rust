$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (17_using_file_using_first)
ON ERROR GOTO h
OPEN "v30_x17_using_file_using_first.tmp" FOR BINARY AS #1
PRINT USING #1, "##"; 5
CLOSE
KILL "v30_x17_using_file_using_first.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
