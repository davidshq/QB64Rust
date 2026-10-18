$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): $COLOR:0, its constants used plainly, with their suffix and with
' another, in a SUB and in a CONST
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
