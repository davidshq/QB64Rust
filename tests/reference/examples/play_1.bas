$CONSOLE:ONLY
ON ERROR GOTO trap
PRINT "Nothing queued:"; PLAY(0)
PLAY "MB V0"
b# = PLAY(0): PLAY "T120 L4 C D E": a# = PLAY(0)
PRINT "Three quarter notes at T120:"; CINT((a# - b#) * 10); "tenths of a second"
PLAY "H"
END
trap:
PRINT "error"; ERR
RESUME NEXT
