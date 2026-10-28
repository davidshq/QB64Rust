$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STRIG.
TYPE t
    strig AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.strig = 4: o.other = 2
o.strig = o.strig + o.other
PRINT o.strig; o.other
SYSTEM
