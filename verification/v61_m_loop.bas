$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOOP.
TYPE t
    loop AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.loop = 4: o.other = 2
o.loop = o.loop + o.other
PRINT o.loop; o.other
SYSTEM
