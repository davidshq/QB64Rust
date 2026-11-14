$CONSOLE:ONLY
a = 1: b = 2
SWAP a, b
PRINT "a ="; a; "and b ="; b
DIM names(1 TO 2) AS STRING
names(1) = "Ney": names(2) = "Murat"
SWAP names(1), names(2)
PRINT names(1); " "; names(2)
DIM code AS STRING * 3
s$ = "long one": code = "abc"
SWAP s$, code
PRINT "["; s$; "] ["; code; "]"
END
