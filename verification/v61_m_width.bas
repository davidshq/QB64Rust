$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WIDTH.
TYPE t
    width AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.width = 4: o.other = 2
o.width = o.width + o.other
PRINT o.width; o.other
SYSTEM
