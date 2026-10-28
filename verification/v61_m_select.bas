$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SELECT.
TYPE t
    select AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.select = 4: o.other = 2
o.select = o.select + o.other
PRINT o.select; o.other
SYSTEM
