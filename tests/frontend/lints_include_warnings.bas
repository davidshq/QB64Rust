' TEST: warnings
' Our lints only for the file being compiled (spec `compiler/lints`, "Only the file being compiled"): the toolbox's
' four routines the program does not call are not reported; its own SUB `Spare` is. QB64pe's "Unused variable" is
' given in included files too, as QB64pe gives it: the toolbox's parameters `x` and `y`.
$CONSOLE:ONLY
MouseShow
'$INCLUDE:'inc/lints_toolbox.bi'
SUB Spare
END SUB
