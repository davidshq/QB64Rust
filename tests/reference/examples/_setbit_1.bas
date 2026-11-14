$CONSOLE:ONLY
flags = 0
flags = _SETBIT(flags, 0)
flags = _SETBIT(flags, 2)
PRINT "flags ="; flags
PRINT "Bit 2 is set:"; _READBIT(flags, 2)
flags = _RESETBIT(flags, 0)
PRINT "After _RESETBIT bit 0:"; flags
PRINT "_TOGGLEBIT(5, 1) ="; _TOGGLEBIT(5, 1)
END
