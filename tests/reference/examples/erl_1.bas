$CONSOLE:ONLY
ON ERROR GOTO handler
10 PRINT "Line 10"
20 x = LOG(0)
30 PRINT "Line 30"
END

handler:
PRINT "Error"; ERR; "after line"; ERL
RESUME NEXT
