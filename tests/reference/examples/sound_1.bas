$CONSOLE:ONLY
ON ERROR GOTO trap
PLAY "MB V0"
b# = PLAY(0): SOUND 440, 18.2, 0: a# = PLAY(0)
PRINT "SOUND 440, 18.2 queued"; CINT((a# - b#) * 10); "tenths of a second"
SOUND 19, 1
SOUND 440, -1
END
trap:
PRINT "error"; ERR
RESUME NEXT
