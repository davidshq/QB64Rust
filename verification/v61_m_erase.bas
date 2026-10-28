$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ERASE.
TYPE t
    erase AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.erase = 4: o.other = 2
o.erase = o.erase + o.other
PRINT o.erase; o.other
SYSTEM
