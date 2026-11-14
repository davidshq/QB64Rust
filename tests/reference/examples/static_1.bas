$CONSOLE:ONLY
FOR i = 1 TO 3
    Visit
NEXT i
END

SUB Visit
    STATIC count AS INTEGER
    DIM fresh AS INTEGER
    count = count + 1
    fresh = fresh + 1
    PRINT "count ="; count; "and fresh ="; fresh
END SUB
