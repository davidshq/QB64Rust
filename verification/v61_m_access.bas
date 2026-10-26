$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ACCESS.
TYPE t
    access AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.access = 4: o.other = 2
o.access = o.access + o.other
PRINT o.access; o.other
SYSTEM
