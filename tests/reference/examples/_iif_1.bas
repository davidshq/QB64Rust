$CONSOLE:ONLY
score = 75
PRINT _IIF(score >= 60, "pass", "fail")
lives = 0
PRINT "Lives left:"; _IIF(lives > 0, lives, 0)
PRINT _IIF(0.4, 1, 2); _IIF(0.6, 1, 2)
END
