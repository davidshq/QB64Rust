$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): $COLOR:32, its constants used plainly, with another suffix and in a
' CONST
$COLOR:32
PRINT HEX$(Red); " "; HEX$(Green); " "; HEX$(Blue); " "; HEX$(AliceBlue); " "; HEX$(YellowGreen)
PRINT Red&; Red~&; Red + 0
CONST mine = Black + 1
PRINT mine
SYSTEM
