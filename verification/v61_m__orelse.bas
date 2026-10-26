$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _ORELSE.
TYPE t
    _orelse AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._orelse = 4: o.other = 2
o._orelse = o._orelse + o.other
PRINT o._orelse; o.other
SYSTEM
