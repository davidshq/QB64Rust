DIM s$ AS STRING
s$ = "test"
SELECT CASE s$
    CASE IS < "m"
        PRINT "Less than m"
    CASE IS >= "m"
        PRINT "m or more"
END SELECT
