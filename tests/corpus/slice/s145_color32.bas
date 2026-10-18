$CONSOLE:ONLY
' $COLOR:32 (m3-constant-evaluation; verification\v45_b_color32): its constants used plainly, with another suffix
' and in a CONST
$COLOR:32
PRINT HEX$(Red); " "; HEX$(Green); " "; HEX$(Blue); " "; HEX$(AliceBlue); " "; HEX$(YellowGreen)
PRINT Red&; Red~&; Red + 0
CONST mine = Black + 1
PRINT mine
SYSTEM
