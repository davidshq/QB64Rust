$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named XOR.
TYPE t
    xor AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.xor = 4: o.other = 2
o.xor = o.xor + o.other
PRINT o.xor; o.other
SYSTEM
