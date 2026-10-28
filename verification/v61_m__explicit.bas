$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _EXPLICIT.
TYPE t
    _explicit AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._explicit = 4: o.other = 2
o._explicit = o._explicit + o.other
PRINT o._explicit; o.other
SYSTEM
