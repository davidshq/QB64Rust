$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _BYTE.
TYPE t
    _byte AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._byte = 4: o.other = 2
o._byte = o._byte + o.other
PRINT o._byte; o.other
SYSTEM
