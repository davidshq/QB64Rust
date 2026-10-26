$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INKEY.
TYPE t
    inkey AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.inkey = 4: o.other = 2
o.inkey = o.inkey + o.other
PRINT o.inkey; o.other
SYSTEM
