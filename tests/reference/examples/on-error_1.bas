$CONSOLE:ONLY
ON ERROR GOTO handler
PRINT "Opening a file that does not exist..."
OPEN "no such file.txt" FOR INPUT AS #1
PRINT "Going on after the error."
ON ERROR GOTO 0
END

handler:
PRINT "Error"; ERR; "was trapped."
RESUME NEXT
