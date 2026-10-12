' TEST: typed
$CONSOLE:ONLY
' Included files (m2-parser-breadth task 8.1, design D9): a $LET in an included file reaches this one, $INCLUDEONCE
' makes a second inclusion empty, a guarded self-include, a file found through the compiler root, a shared
' variable and SUBs from included files.
'$INCLUDE:'inc/lib.bi'
'$INCLUDE:'inc/once.bi'
REM $INCLUDE: 'inc/once.bi'
'$INCLUDE:'inc/self_guarded.bi'
'$INCLUDE:'rootonly.bi'
$IF FAST = 1 THEN
    PRINT LIBK; ONCEK; GUARDK; ROOTK
$END IF
shared_n = 5
show shared_n
SYSTEM
'$INCLUDE:'inc/subs.bm'
