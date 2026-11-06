$CONSOLE:ONLY
' v68 "Unused variable" with an included file: `frommain` is used only there, `inlib` is declared there and never
' used, and `usedlib` is declared there and used here.
DIM frommain AS INTEGER
'$INCLUDE:'v68_d_include.bi'
PRINT usedlib
SYSTEM
