$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named UNTIL.
TYPE t
    until AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.until = 4: o.other = 2
o.until = o.until + o.other
PRINT o.until; o.other
SYSTEM
