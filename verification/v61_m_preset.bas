$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PRESET.
TYPE t
    preset AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.preset = 4: o.other = 2
o.preset = o.preset + o.other
PRINT o.preset; o.other
SYSTEM
