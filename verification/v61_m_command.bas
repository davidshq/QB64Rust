$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named COMMAND.
TYPE t
    command AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.command = 4: o.other = 2
o.command = o.command + o.other
PRINT o.command; o.other
SYSTEM
