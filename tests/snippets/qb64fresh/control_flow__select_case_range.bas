DIM x AS LONG
x = 50
SELECT CASE x
    CASE 1 TO 10
        PRINT "1-10"
    CASE 11 TO 100
        PRINT "11-100"
    CASE ELSE
        PRINT "Other"
END SELECT
