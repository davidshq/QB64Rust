$CONSOLE:ONLY
' OPTION _EXPLICIT only in an included file, with the undeclared variable used before the include line.
undeclared = 2
'$INCLUDE:'v19_inc/explicit.bi'
declared = 1
PRINT declared; undeclared
SYSTEM
