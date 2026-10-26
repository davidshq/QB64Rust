$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SADD.
TYPE t
    sadd AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sadd = 4: o.other = 2
o.sadd = o.sadd + o.other
PRINT o.sadd; o.other
SYSTEM
