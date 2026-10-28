$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named IF.
TYPE t
    if AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.if = 4: o.other = 2
o.if = o.if + o.other
PRINT o.if; o.other
SYSTEM
