$CONSOLE:ONLY
n = 1
DO WHILE n < 100
    n = n * 3
LOOP
PRINT "First power of 3 from 100 up:"; n
tries = 0
DO
    tries = tries + 1
LOOP UNTIL tries >= 4
PRINT "tries ="; tries
DO
    m = m + 1
    IF m = 3 THEN EXIT DO
LOOP
PRINT "left the loop at m ="; m
END
