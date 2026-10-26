$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DATA.
TYPE t
    data AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.data = 4: o.other = 2
o.data = o.data + o.other
PRINT o.data; o.other
SYSTEM
