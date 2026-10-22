$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x46_var_string_array_dollar)
' name$(i) of an AS STRING array.
DIM a(3) AS STRING
a$(1) = "q"
PRINT a(1)
SYSTEM
