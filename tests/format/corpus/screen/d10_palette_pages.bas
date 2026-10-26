' PALETTE with and without arguments, PCOPY, and SCREEN with its page arguments.
Screen 7, , 1, 0
Print "page one"
Line (10, 20)-(50, 40), 4, BF
PCopy 1, 2
Screen 7, , 2, 0
PSet (5, 100), 14
Screen , , 0, 0
Print "page zero"
Palette 2, 5
Palette
Palette 1, 12
Palette 4, 9
PCopy 0, 3
Screen , , 3
Print
Print "page three"
Screen , , , 1
System
