$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ATN.
TYPE t
    atn AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.atn = 4: o.other = 2
o.atn = o.atn + o.other
PRINT o.atn; o.other
SYSTEM
