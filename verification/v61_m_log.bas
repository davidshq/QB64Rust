$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOG.
TYPE t
    log AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.log = 4: o.other = 2
o.log = o.log + o.other
PRINT o.log; o.other
SYSTEM
