$CONSOLE:ONLY
OPEN "lock_1.tmp" FOR BINARY AS #1
s$ = "abcdefgh"
PUT #1, , s$
LOCK #1, 1 TO 4
PRINT "Bytes 1 to 4 are locked."
UNLOCK #1, 1 TO 4
PRINT "and unlocked again."
CLOSE #1
KILL "lock_1.tmp"
END
