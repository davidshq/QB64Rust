$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OPEN.
TYPE t
    open AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.open = 4: o.other = 2
o.open = o.open + o.other
PRINT o.open; o.other
SYSTEM
