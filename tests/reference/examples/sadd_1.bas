$CONSOLE:ONLY
DIM s AS STRING
s = "hello"
DEF SEG = VARSEG(s)
PRINT "The first and last characters:"; PEEK(SADD(s)); PEEK(SADD(s) + 4)
DEF SEG
END
