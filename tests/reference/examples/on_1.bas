$CONSOLE:ONLY
FOR choice = 1 TO 4
    ON choice GOSUB first, second, third
NEXT choice
PRINT "4 is beyond the list, so nothing was called for it."
END

first: PRINT "first": RETURN
second: PRINT "second": RETURN
third: PRINT "third": RETURN
