$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named NOT.
TYPE t
    not AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.not = 4: o.other = 2
o.not = o.not + o.other
PRINT o.not; o.other
SYSTEM
