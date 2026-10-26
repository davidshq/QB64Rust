$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _NEGATE.
TYPE t
    _negate AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._negate = 4: o.other = 2
o._negate = o._negate + o.other
PRINT o._negate; o.other
SYSTEM
