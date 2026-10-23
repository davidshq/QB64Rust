$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x47_fixed_array_dollar_preserve)
' REDIM _PRESERVE of a STRING * n array, then name$(i) (QB64pe hash.bas).
REDIM a(1 TO 2) AS STRING * 3
a(2) = "kl"
REDIM _PRESERVE a(1 TO 4) AS STRING * 3
PRINT "["; a$(2); "]"; UBOUND(a$)
SYSTEM
