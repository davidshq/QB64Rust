' TEST: typed
$CONSOLE:ONLY
' A whole array of a name that is no array in file GET and PUT and in _UCHARPOS is the array used without DIM
' (m4-memory-records design D6; verification\v30_x176-x178, v31_x01_accepted, v56_e): und() an array of
' SINGLE, unl&() of LONG, nd&() of LONG for _UCHARPOS; GET of und$() reads into its first element as a string
OPEN "a.tmp" FOR BINARY AS #1
GET #1, 1, und()
PUT #1, , und()
GET #1, , unl&()
GET #1, , und$()
CLOSE #1
n = _UCHARPOS("abc", nd&())
SYSTEM
