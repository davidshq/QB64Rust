$CONSOLE:ONLY
DIM names(1 TO 3) AS STRING
names(1) = "Ney": names(2) = "Murat": names(3) = "Soult"
FOR i = LBOUND(names) TO UBOUND(names)
    PRINT i; names(i)
NEXT i
END
