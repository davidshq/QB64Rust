$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SCREEN.
TYPE t
    screen AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.screen = 4: o.other = 2
o.screen = o.screen + o.other
PRINT o.screen; o.other
SYSTEM
