$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEFDBL.
TYPE t
    defdbl AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.defdbl = 4: o.other = 2
o.defdbl = o.defdbl + o.other
PRINT o.defdbl; o.other
SYSTEM
