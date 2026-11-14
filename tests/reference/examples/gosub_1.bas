$CONSOLE:ONLY
x = 3: GOSUB square
x = 5: GOSUB square
PRINT "Back in the main program."
END

square:
PRINT x; "squared is"; x * x
RETURN
