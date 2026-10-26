$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _BIT.
TYPE t
    _bit AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._bit = 4: o.other = 2
o._bit = o._bit + o.other
PRINT o._bit; o.other
SYSTEM
