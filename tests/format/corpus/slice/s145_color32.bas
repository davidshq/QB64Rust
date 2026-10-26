$Console:Only
' $COLOR:32 (m3-constant-evaluation; verification\v45_b_color32): its constants used plainly, with another suffix
' and in a CONST
$Color:32
Print Hex$(Red); " "; Hex$(Green); " "; Hex$(Blue); " "; Hex$(AliceBlue); " "; Hex$(YellowGreen)
Print Red&; Red~&; Red + 0
Const mine = Black + 1
Print mine
System
