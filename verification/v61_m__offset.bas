$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _OFFSET.
TYPE t
    _offset AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._offset = 4: o.other = 2
o._offset = o._offset + o.other
PRINT o._offset; o.other
SYSTEM
