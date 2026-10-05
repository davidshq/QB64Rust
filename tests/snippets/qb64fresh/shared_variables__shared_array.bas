DIM scores(10) AS LONG
scores(0) = 100
CALL UpdateScore(0, 200)
PRINT scores(0)
END

SUB UpdateScore(index AS LONG, value AS LONG)
    SHARED scores()
    scores(index) = value
END SUB
