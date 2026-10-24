$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): the stub CALL CLOSE.
OPEN "v57_s12.tmp" FOR OUTPUT AS #1
CALL CLOSE
PRINT "open:"; FREEFILE
KILL "v57_s12.tmp"
PRINT "ok"
SYSTEM
