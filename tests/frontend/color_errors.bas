' TEST: check-fail
$CONSOLE:ONLY
' $COLOR rejected (m3-constant-evaluation task 4.1, design D8), each form one program of verification v45_x45-x59
' where the old compiler rejects it too: a CONST above the first $COLOR line that names a colour (the first pass
' has not included the colour file there), the other set after one, a mode other than 0 and 32, a word or a
' blank in the line, $COLOR alone, a variable, a SUB or a CONST with a colour's name, a store into a colour, a
' colour with $.
CONST early = Black + 1
$COLOR:0
$COLOR:32
$COLOR:16
$COLOR:0 x
$COLOR: 0
$COLOR
DIM Blue AS INTEGER
CONST Red = 5
Green = 3
PRINT Cyan$
SYSTEM

SUB Yellow
END SUB
