$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named TAN.
TYPE t
    tan AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.tan = 4: o.other = 2
o.tan = o.tan + o.other
PRINT o.tan; o.other
SYSTEM
