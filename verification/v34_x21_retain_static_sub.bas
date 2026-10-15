$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (21_retain_static_sub)
s
s
SYSTEM
SUB s STATIC
    REDIM _RETAIN a(1 TO 2) AS LONG
    a(1) = a(1) + 1
    PRINT a(1)
END SUB
