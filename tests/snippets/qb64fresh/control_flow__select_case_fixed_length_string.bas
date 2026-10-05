DIM s AS STRING * 10
s = "test"
SELECT CASE s
    CASE "test"
        PRINT "Match"
    CASE ELSE
        PRINT "No match"
END SELECT
