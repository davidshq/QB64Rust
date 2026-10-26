$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named COM.
TYPE t
    com AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.com = 4: o.other = 2
o.com = o.com + o.other
PRINT o.com; o.other
SYSTEM
