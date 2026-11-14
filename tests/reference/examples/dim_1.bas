$CONSOLE:ONLY
DIM total AS LONG, name1 AS STRING
DIM AS DOUBLE x, y
DIM scores(1 TO 5) AS INTEGER
DIM grid(3, 3)
total = 100000: name1 = "Davout": x = 1 / 3
scores(1) = 90: scores(5) = 70
grid(3, 3) = 2.5
PRINT name1; total; x; y
PRINT "scores runs from"; LBOUND(scores); "to"; UBOUND(scores); "and scores(5) is"; scores(5)
PRINT "grid(3, 3) is"; grid(3, 3); "and grid(0, 0) is"; grid(0, 0)
END
