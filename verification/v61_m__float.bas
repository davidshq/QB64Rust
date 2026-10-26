$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _FLOAT.
TYPE t
    _float AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._float = 4: o.other = 2
o._float = o._float + o.other
PRINT o._float; o.other
SYSTEM
