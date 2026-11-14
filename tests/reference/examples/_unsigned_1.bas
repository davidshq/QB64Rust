$CONSOLE:ONLY
DIM u AS _UNSIGNED INTEGER
u = 65535: PRINT "_UNSIGNED INTEGER 65535:"; u
u = -1: PRINT "-1 stored:"; u
DIM s AS INTEGER
s = -1: PRINT "-1 in a signed INTEGER:"; s
v~& = 4294967295: PRINT "With the suffix ~&:"; v~&
END
