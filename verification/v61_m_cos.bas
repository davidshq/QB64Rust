$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named COS.
TYPE t
    cos AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.cos = 4: o.other = 2
o.cos = o.cos + o.other
PRINT o.cos; o.other
SYSTEM
