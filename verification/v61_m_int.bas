$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INT.
TYPE t
    int AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.int = 4: o.other = 2
o.int = o.int + o.other
PRINT o.int; o.other
SYSTEM
