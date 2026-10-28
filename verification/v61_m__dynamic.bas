$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _DYNAMIC.
TYPE t
    _dynamic AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._dynamic = 4: o.other = 2
o._dynamic = o._dynamic + o.other
PRINT o._dynamic; o.other
SYSTEM
