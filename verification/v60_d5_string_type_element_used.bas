$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR of a member of an element of an array of a TYPE with a STRING
' member, then the element's string used (an array is not in conventional memory).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM a(2) AS ts
PRINT VARPTR(a(1).v)
a(1).n = "xy"
PRINT a(1).n; LEN(a(1).n)
SYSTEM
