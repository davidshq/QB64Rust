$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named EXP.
TYPE t
    exp AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.exp = 4: o.other = 2
o.exp = o.exp + o.other
PRINT o.exp; o.other
SYSTEM
