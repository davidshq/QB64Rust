$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 4.4): an array in conventional memory that is larger than a segment
' (80,000 bytes against 65,536): what the program does at its start.
DIM big(1 TO 40000) AS INTEGER, l AS LONG
PRINT "start"
l = VARSEG(big(1))
big(40000) = 7
PRINT "after"; big(40000)
SYSTEM
