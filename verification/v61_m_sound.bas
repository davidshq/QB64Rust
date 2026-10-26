$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SOUND.
TYPE t
    sound AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sound = 4: o.other = 2
o.sound = o.sound + o.other
PRINT o.sound; o.other
SYSTEM
