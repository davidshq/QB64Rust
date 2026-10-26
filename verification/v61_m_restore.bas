$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RESTORE.
TYPE t
    restore AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.restore = 4: o.other = 2
o.restore = o.restore + o.other
PRINT o.restore; o.other
SYSTEM
