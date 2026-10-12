$CONSOLE:ONLY
' OPTION _EXPLICIT standing only in an included file (m2-parser-breadth task 8.1): does it apply to the main
' file's undeclared variable after the include?
'$INCLUDE:'v19_inc/explicit.bi'
declared = 1
undeclared = 2
PRINT declared; undeclared
SYSTEM
