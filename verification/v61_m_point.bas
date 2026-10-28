$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named POINT.
TYPE t
    point AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.point = 4: o.other = 2
o.point = o.point + o.other
PRINT o.point; o.other
SYSTEM
