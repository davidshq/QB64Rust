$CONSOLE:ONLY
RANDOMIZE USING 42
a = RND: b = RND
RANDOMIZE USING 42
PRINT "The same seed gives the same values:"; a = RND; b = RND
RANDOMIZE TIMER
x = RND
PRINT "A value from 0 up to 1:"; x >= 0 AND x < 1
END
