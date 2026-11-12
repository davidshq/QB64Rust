$CONSOLE:ONLY
DIM b(10) AS INTEGER, m AS _MEM
m = _MEM(b())
PRINT "The whole array:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
m = _MEM(b(4))
PRINT "From b(4) to the end:"; m.SIZE
m = _MEMELEMENT(b(4))
PRINT "b(4) alone:"; m.SIZE
END
