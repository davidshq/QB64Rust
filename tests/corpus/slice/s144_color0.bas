$CONSOLE:ONLY
' $COLOR:0 (m3-constant-evaluation; verification\v45_b_color0): its constants used plainly, with their suffix and
' with another, in a SUB and in a CONST
$COLOR:0
CONST newGreen = Green
PRINT Black; Blue; Green; Cyan; Red; Magenta; Brown; White; Gray; LightBlue; LightGreen; LightCyan; LightRed
PRINT LightMagenta; Yellow; BrightWhite; newGreen
PRINT Red~%%; Red%; Red&&; Red + 0
s
SUB s
    PRINT Yellow
END SUB
SYSTEM
