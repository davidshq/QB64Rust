$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named POKE.
TYPE t
    poke AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.poke = 4: o.other = 2
o.poke = o.poke + o.other
PRINT o.poke; o.other
SYSTEM
