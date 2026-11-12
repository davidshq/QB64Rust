$CONSOLE:ONLY
n = 0
DO WHILE _MOUSEINPUT
    n = n + 1
LOOP
PRINT "Mouse events waiting:"; n
END
