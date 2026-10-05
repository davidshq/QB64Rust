PRINT GCD%(48, 18)
END

FUNCTION GCD%(a AS INTEGER, b AS INTEGER) AS INTEGER
    IF b = 0 THEN
        GCD% = a
    ELSE
        GCD% = GCD%(b, a MOD b)
    END IF
END FUNCTION
