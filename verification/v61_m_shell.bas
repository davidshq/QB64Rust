$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SHELL.
TYPE t
    shell AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.shell = 4: o.other = 2
o.shell = o.shell + o.other
PRINT o.shell; o.other
SYSTEM
