$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _STATIC.
TYPE t
    _static AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._static = 4: o.other = 2
o._static = o._static + o.other
PRINT o._static; o.other
SYSTEM
