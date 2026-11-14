$CONSOLE:ONLY
ON ERROR GOTO retry
d = -16
PRINT "The square root of"; d; "is"; SQR(d)
PRINT "Done."
END

retry:
PRINT "Error"; ERR; ": using 16 instead."
d = 16
RESUME
