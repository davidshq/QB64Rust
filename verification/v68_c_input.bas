$CONSOLE:ONLY
' v68 "Unused variable" for targets of INPUT and LINE INPUT that are never read again.
INPUT "Press Enter", dummy$
LINE INPUT "Again", other$
INPUT "Two", first, second
PRINT second
SYSTEM
