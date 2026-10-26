$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _UNSIGNED.
TYPE t
    _unsigned AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._unsigned = 4: o.other = 2
o._unsigned = o._unsigned + o.other
PRINT o._unsigned; o.other
SYSTEM
