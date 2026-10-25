$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): VARPTR of the STRING member of an element of an array of a TYPE.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM e(2) AS ts
PRINT VARPTR(e(2).n)
SYSTEM
