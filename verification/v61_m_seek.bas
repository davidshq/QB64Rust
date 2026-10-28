$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SEEK.
TYPE t
    seek AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.seek = 4: o.other = 2
o.seek = o.seek + o.other
PRINT o.seek; o.other
SYSTEM
