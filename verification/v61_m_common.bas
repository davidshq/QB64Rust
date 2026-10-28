$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named COMMON.
TYPE t
    common AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.common = 4: o.other = 2
o.common = o.common + o.other
PRINT o.common; o.other
SYSTEM
