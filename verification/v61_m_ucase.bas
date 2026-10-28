$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named UCASE.
TYPE t
    ucase AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.ucase = 4: o.other = 2
o.ucase = o.ucase + o.other
PRINT o.ucase; o.other
SYSTEM
