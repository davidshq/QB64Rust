$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named REM.
TYPE t
    rem AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.rem = 4: o.other = 2
o.rem = o.rem + o.other
PRINT o.rem; o.other
SYSTEM
