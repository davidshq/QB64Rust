$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LEFT.
TYPE t
    left AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.left = 4: o.other = 2
o.left = o.left + o.other
PRINT o.left; o.other
SYSTEM
