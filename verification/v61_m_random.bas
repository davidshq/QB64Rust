$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RANDOM.
TYPE t
    random AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.random = 4: o.other = 2
o.random = o.random + o.other
PRINT o.random; o.other
SYSTEM
