$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LCASE.
TYPE t
    lcase AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.lcase = 4: o.other = 2
o.lcase = o.lcase + o.other
PRINT o.lcase; o.other
SYSTEM
