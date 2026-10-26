'Test to prove auto-including of $COLOR:32 constants on demand.

$Console:Only
$Color:32

'Let's also try to re-assign a color CONST to a new CONST to prove
'availability of the color constants during pre-pass
Const newGreen = Green

Print "Some $COLOR:32 (color32.bi) constants:"
Print "--------------------------------------"
Print "Red.....:"; Red
Print "Green...:"; newGreen
Print "Blue....:"; Blue

System

