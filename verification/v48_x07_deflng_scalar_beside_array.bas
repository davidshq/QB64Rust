$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): a plain scalar beside an array typed by REDIM SHARED ... AS INTEGER under DEFLNG.
DEFLNG A-Z
REDIM SHARED t(1 TO 8) AS INTEGER
t = 100000: t(1) = 77
PRINT t; t(1); LEN(t); LEN(t(1))
show
SYSTEM

SUB show
    t = t + 1
    PRINT t; t(1); LEN(t)
END SUB
