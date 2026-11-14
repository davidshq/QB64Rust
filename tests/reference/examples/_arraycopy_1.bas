$CONSOLE:ONLY
DIM src(1 TO 5) AS LONG, dst(0 TO 9) AS LONG
FOR i = 1 TO 5: src(i) = i * 10: NEXT i
_ARRAYCOPY src(2 TO 4) TO dst(0)
PRINT dst(0); dst(1); dst(2); dst(3)
_ARRAYCOPY src() TO dst(5)
PRINT dst(5); dst(9)
END
