$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PALETTE.
TYPE t
    palette AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.palette = 4: o.other = 2
o.palette = o.palette + o.other
PRINT o.palette; o.other
SYSTEM
