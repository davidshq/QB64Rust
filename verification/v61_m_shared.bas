$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SHARED.
TYPE t
    shared AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.shared = 4: o.other = 2
o.shared = o.shared + o.other
PRINT o.shared; o.other
SYSTEM
