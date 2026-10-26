$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _INTEGER64.
TYPE t
    _integer64 AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._integer64 = 4: o.other = 2
o._integer64 = o._integer64 + o.other
PRINT o._integer64; o.other
SYSTEM
