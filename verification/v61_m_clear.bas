$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CLEAR.
TYPE t
    clear AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.clear = 4: o.other = 2
o.clear = o.clear + o.other
PRINT o.clear; o.other
SYSTEM
