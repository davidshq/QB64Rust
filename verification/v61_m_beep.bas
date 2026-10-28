$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named BEEP.
TYPE t
    beep AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.beep = 4: o.other = 2
o.beep = o.beep + o.other
PRINT o.beep; o.other
SYSTEM
