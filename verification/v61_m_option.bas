$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OPTION.
TYPE t
    option AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.option = 4: o.other = 2
o.option = o.option + o.other
PRINT o.option; o.other
SYSTEM
