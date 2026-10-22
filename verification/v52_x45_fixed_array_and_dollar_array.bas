$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x45_fixed_array_and_dollar_array)
' A STRING * n array and an array name$ of its own.
DIM a(3) AS STRING * 4
DIM a$(3)
a(1) = "f": a$(1) = "v"
PRINT "["; a(1); "]["; a$(1); "]"
SYSTEM
