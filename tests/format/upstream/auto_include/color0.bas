'Test to prove auto-including of $COLOR:0 constants on demand.

$Console:Only
$Color:0

'Let's also try to re-assign a color CONST to a new CONST to prove
'availability of the color constants during pre-pass
Const newGreen = Green

Print "Some $COLOR:0 (color0.bi) constants:"
Print "------------------------------------"
Print "Red.....:"; Red
Print "Green...:"; newGreen
Print "Blue....:"; Blue

System

