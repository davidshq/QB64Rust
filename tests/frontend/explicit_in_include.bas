' TEST: check-fail
$CONSOLE:ONLY
' The only OPTION _EXPLICIT stands in an included file: it applies to the whole program, also before the include
' line (measured, verification\v19_explicit_in_include and v19_explicit_before_include: "Variable 'undeclared'
' (SINGLE) not defined").
undeclared = 2
'$INCLUDE:'inc/explicit.bi'
declared = 1
PRINT declared; undeclared
