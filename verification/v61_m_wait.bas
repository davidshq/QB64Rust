$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WAIT.
TYPE t
    wait AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.wait = 4: o.other = 2
o.wait = o.wait + o.other
PRINT o.wait; o.other
SYSTEM
