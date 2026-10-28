$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named EXIT.
TYPE t
    exit AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.exit = 4: o.other = 2
o.exit = o.exit + o.other
PRINT o.exit; o.other
SYSTEM
