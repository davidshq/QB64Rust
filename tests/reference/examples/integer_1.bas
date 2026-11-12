$CONSOLE:ONLY
DIM score AS INTEGER
score = 32767
PRINT "score ="; score; "takes"; LEN(score); "bytes"
score = 2.5: PRINT "2.5 stored:"; score
score = 3.5: PRINT "3.5 stored:"; score
n% = 70000: PRINT "70000 stored:"; n%
score = 32767: PRINT "score + 1 in an expression:"; score + 1
END
