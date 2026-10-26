$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PUT.
TYPE t
    put AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.put = 4: o.other = 2
o.put = o.put + o.other
PRINT o.put; o.other
SYSTEM
