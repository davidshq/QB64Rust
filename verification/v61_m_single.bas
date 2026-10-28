$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SINGLE.
TYPE t
    single AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.single = 4: o.other = 2
o.single = o.single + o.other
PRINT o.single; o.other
SYSTEM
