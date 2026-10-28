$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ABS.
TYPE t
    abs AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.abs = 4: o.other = 2
o.abs = o.abs + o.other
PRINT o.abs; o.other
SYSTEM
