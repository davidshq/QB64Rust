$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named REDIM.
TYPE t
    redim AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.redim = 4: o.other = 2
o.redim = o.redim + o.other
PRINT o.redim; o.other
SYSTEM
