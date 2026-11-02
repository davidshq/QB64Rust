$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-qb64pe-sources row on the $ name of a STRING * n array): STATIC a$(3) in a SUB beside its STRING * 4 array a.
s
SYSTEM

SUB s
    DIM a(3) AS STRING * 4
    STATIC a$(3)
    a$(1) = "xy"
    PRINT LEN(a(1)); "["; a$(1); "]"
END SUB
