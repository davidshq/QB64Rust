$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CIRCLE.
TYPE t
    circle AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.circle = 4: o.other = 2
o.circle = o.circle + o.other
PRINT o.circle; o.other
SYSTEM
