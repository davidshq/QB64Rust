' PALETTE with and without arguments, PCOPY, and SCREEN with its page arguments.
SCREEN 7, , 1, 0
PRINT "page one"
LINE (10, 20)-(50, 40), 4, BF
PCOPY 1, 2
SCREEN 7, , 2, 0
PSET (5, 100), 14
SCREEN , , 0, 0
PRINT "page zero"
PALETTE 2, 5
PALETTE
PALETTE 1, 12
PALETTE 4, 9
PCOPY 0, 3
SCREEN , , 3
PRINT
PRINT "page three"
SCREEN , , , 1
SYSTEM
