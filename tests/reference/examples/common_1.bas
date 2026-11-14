$CONSOLE:ONLY
COMMON SHARED level AS INTEGER, names() AS STRING
DIM names(1 TO 2) AS STRING
level = 3
names(1) = "Lannes": names(2) = "Soult"
Show
END

SUB Show
    PRINT "level"; level; names(1); " and "; names(2)
END SUB
