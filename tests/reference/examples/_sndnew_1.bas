$CONSOLE:ONLY
n = _SNDNEW(800, 1, 16, 8000)
DIM m AS _MEM
m = _MEMSOUND(n)
PRINT "Bytes, type, bytes per sample:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
PRINT "Length in milliseconds:"; CINT(_SNDLEN(n) * 1000)
PRINT "Three channels:"; _SNDNEW(1024, 3, 16)
END
