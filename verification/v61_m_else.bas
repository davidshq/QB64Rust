$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ELSE.
TYPE t
    else AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.else = 4: o.other = 2
o.else = o.else + o.other
PRINT o.else; o.other
SYSTEM
