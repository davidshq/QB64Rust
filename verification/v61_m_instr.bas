$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INSTR.
TYPE t
    instr AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.instr = 4: o.other = 2
o.instr = o.instr + o.other
PRINT o.instr; o.other
SYSTEM
