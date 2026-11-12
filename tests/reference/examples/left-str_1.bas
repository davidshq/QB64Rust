$CONSOLE:ONLY
city$ = "Wellington"
PRINT LEFT$(city$, 4)
FOR i = 1 TO 4
    PRINT LEFT$("QB64", i)
NEXT
IF LEFT$(city$, 1) = "W" THEN PRINT "It starts with W."
PRINT "["; LEFT$(city$, 0); "]"; LEN(LEFT$(city$, 99))
END
