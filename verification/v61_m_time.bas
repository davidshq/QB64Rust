$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named TIME.
TYPE t
    time AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.time = 4: o.other = 2
o.time = o.time + o.other
PRINT o.time; o.other
SYSTEM
