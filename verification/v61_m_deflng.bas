$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEFLNG.
TYPE t
    deflng AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.deflng = 4: o.other = 2
o.deflng = o.deflng + o.other
PRINT o.deflng; o.other
SYSTEM
