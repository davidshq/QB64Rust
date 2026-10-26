$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LEN.
TYPE t
    len AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.len = 4: o.other = 2
o.len = o.len + o.other
PRINT o.len; o.other
SYSTEM
