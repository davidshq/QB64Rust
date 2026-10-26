$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OUTPUT.
TYPE t
    output AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.output = 4: o.other = 2
o.output = o.output + o.other
PRINT o.output; o.other
SYSTEM
