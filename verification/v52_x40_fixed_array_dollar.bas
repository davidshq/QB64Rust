$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x40_fixed_array_dollar)
' name$(i) and UBOUND(name$) of a STRING * n array.
REDIM a(1 TO 3) AS STRING * 4
a(2) = "ab"
PRINT "["; a$(2); "]"; UBOUND(a$)
SYSTEM
