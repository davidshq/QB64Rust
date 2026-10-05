DIM s$ AS STRING
s$ = "cat"
SELECT CASE s$
    CASE "a" TO "b"
        PRINT "A-B"
    CASE "c" TO "d"
        PRINT "C-D"
    CASE ELSE
        PRINT "Other"
END SELECT
