$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named Y.
TYPE t
    y AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.y = 4: o.other = 2
o.y = o.y + o.other
PRINT o.y; o.other
SYSTEM
