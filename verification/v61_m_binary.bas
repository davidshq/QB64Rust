$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named BINARY.
TYPE t
    binary AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.binary = 4: o.other = 2
o.binary = o.binary + o.other
PRINT o.binary; o.other
SYSTEM
