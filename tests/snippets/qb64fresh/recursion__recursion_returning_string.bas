PRINT ReverseStr$("Hello")
END

FUNCTION ReverseStr$(s AS STRING) AS STRING
    IF LEN(s) <= 1 THEN
        ReverseStr$ = s
    ELSE
        ReverseStr$ = RIGHT$(s, 1) + ReverseStr$(LEFT$(s, LEN(s) - 1))
    END IF
END FUNCTION
