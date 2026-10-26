$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named UEVENT.
TYPE t
    uevent AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.uevent = 4: o.other = 2
o.uevent = o.uevent + o.other
PRINT o.uevent; o.other
SYSTEM
