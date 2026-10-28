$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DATE.
TYPE t
    date AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.date = 4: o.other = 2
o.date = o.date + o.other
PRINT o.date; o.other
SYSTEM
