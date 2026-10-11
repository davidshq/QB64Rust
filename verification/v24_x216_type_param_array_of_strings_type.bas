$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array of a type with a STRING member passed to an array
' parameter and changed.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(2) AS ts
a(1).n = "one"
change a()
PRINT a(1).n; a(2).n
SYSTEM
SUB change (p() AS ts)
    p(1).n = p(1).n + "!"
    p(2).n = "two"
END SUB
