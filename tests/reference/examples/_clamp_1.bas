$CONSOLE:ONLY
PRINT _CLAMP(15, 0, 10); _CLAMP(-5, 0, 10); _CLAMP(5, 0, 10)
health = 130
health = _CLAMP(health, 0, 100)
PRINT "Health is held at"; health
END
