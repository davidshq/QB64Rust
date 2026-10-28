$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named TO.
TYPE t
    to AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.to = 4: o.other = 2
o.to = o.to + o.other
PRINT o.to; o.other
SYSTEM
