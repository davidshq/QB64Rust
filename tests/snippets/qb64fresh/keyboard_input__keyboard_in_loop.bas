DIM k AS LONG
DO
    k = _KEYHIT
    IF k <> 0 THEN PRINT k
LOOP UNTIL k = 27
