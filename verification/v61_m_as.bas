$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named AS.
TYPE t
    as AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.as = 4: o.other = 2
o.as = o.as + o.other
PRINT o.as; o.other
SYSTEM
