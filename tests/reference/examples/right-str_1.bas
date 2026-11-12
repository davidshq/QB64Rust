$CONSOLE:ONLY
PRINT RIGHT$("Wellington", 3)
PRINT "["; RIGHT$("abc", 0); "] ["; RIGHT$("abc", 5); "]"
file$ = "report.txt"
PRINT "Extension: "; RIGHT$(file$, 3)
END
