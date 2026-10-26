$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RIGHT.
TYPE t
    right AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.right = 4: o.other = 2
o.right = o.right + o.other
PRINT o.right; o.other
SYSTEM
