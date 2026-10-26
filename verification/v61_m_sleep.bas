$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SLEEP.
TYPE t
    sleep AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sleep = 4: o.other = 2
o.sleep = o.sleep + o.other
PRINT o.sleep; o.other
SYSTEM
