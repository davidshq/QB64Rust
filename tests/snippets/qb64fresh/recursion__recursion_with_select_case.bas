PRINT Collatz(7)
END

FUNCTION Collatz(n AS LONG) AS LONG
    SELECT CASE n
        CASE 1
            Collatz = 0
        CASE ELSE
            IF n MOD 2 = 0 THEN
                Collatz = 1 + Collatz(n \ 2)
            ELSE
                Collatz = 1 + Collatz(3 * n + 1)
            END IF
    END SELECT
END FUNCTION
