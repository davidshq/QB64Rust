$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x42_fixed_array_dollar_store)
' name$(i) as a place of a STRING * n array.
DIM a(3) AS STRING * 2
a$(1) = "zzz"
PRINT "["; a(1); "]"
SYSTEM
