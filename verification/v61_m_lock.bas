$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOCK.
TYPE t
    lock AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.lock = 4: o.other = 2
o.lock = o.lock + o.other
PRINT o.lock; o.other
SYSTEM
