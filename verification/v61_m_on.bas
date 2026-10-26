$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ON.
TYPE t
    on AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.on = 4: o.other = 2
o.on = o.on + o.other
PRINT o.on; o.other
SYSTEM
