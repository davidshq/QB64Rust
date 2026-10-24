$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON only in an included file (v58_inc\e_on.bi), the recursion in the
' main file: the setting holds for the whole program.
'$INCLUDE:'v58_inc/e_on.bi'
PRINT "start"
deep 1
SYSTEM

SUB deep (n AS LONG)
    deep n + 1
END SUB
