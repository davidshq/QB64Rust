$CONSOLE:ONLY
' Verification (m2-numeric-types, task 1.6): &H, &O and &B literals with digits wider than their suffix's type:
' the value PRINT shows and the value an operation sees (v21_a_literals: &H1FF~%% is 255 printed, 511 in + 0;
' &HFF%% is -1 everywhere). Signed ones wider than their type are rejected ("Overflow", v21_x37 to x41).
ON ERROR GOTO h
PRINT "print:"; &H1FF~%%; &H1FFFF~%; &H1FFFFFFFF~&; &O777~%%; &B111111111~%%
PRINT "+ 0:"; &H1FF~%% + 0; &H1FFFF~% + 0; &H1FFFFFFFF~& + 0; &O777~%% + 0; &B111111111~%% + 0
PRINT "fit + 0:"; &HFF%% + 0; &HFFFF% + 0; &HFFFFFFFF& + 0; &H80%% + 0; &H7F%% + 0; &HFF~%% + 0
x& = &H1FF~%%: y& = &HFF%%
PRINT "store:"; x&; y&
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
