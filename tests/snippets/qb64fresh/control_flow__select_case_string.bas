DIM s$ AS STRING
s$ = "two"
SELECT CASE s$
    CASE "one"
        PRINT "One"
    CASE "two"
        PRINT "Two"
    CASE "three"
        PRINT "Three"
    CASE ELSE
        PRINT "Other"
END SELECT
