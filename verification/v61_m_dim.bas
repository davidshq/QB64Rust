$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DIM.
TYPE t
    dim AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.dim = 4: o.other = 2
o.dim = o.dim + o.other
PRINT o.dim; o.other
SYSTEM
