$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named NAME.
TYPE t
    name AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.name = 4: o.other = 2
o.name = o.name + o.other
PRINT o.name; o.other
SYSTEM
