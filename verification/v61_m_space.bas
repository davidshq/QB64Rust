$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SPACE.
TYPE t
    space AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.space = 4: o.other = 2
o.space = o.space + o.other
PRINT o.space; o.other
SYSTEM
