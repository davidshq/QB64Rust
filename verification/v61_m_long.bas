$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LONG.
TYPE t
    long AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.long = 4: o.other = 2
o.long = o.long + o.other
PRINT o.long; o.other
SYSTEM
