$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _ANDALSO.
TYPE t
    _andalso AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._andalso = 4: o.other = 2
o._andalso = o._andalso + o.other
PRINT o._andalso; o.other
SYSTEM
