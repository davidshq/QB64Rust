$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named X.
TYPE t
    x AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.x = 4: o.other = 2
o.x = o.x + o.other
PRINT o.x; o.other
SYSTEM
