$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x41_fixed_array_dollar_sub)
' name$(i) in a SUB of a DIM SHARED STRING * n array.
DIM SHARED h(5) AS STRING * 8
s
SYSTEM
SUB s
    h(1) = "x"
    PRINT LEN(h$(1)); "["; h$(1); "]"
END SUB
