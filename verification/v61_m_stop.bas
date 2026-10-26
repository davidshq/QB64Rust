$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STOP.
TYPE t
    stop AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.stop = 4: o.other = 2
o.stop = o.stop + o.other
PRINT o.stop; o.other
SYSTEM
