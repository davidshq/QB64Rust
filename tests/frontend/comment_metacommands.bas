' TEST: check-fail
$CONSOLE:ONLY
' change m2-parser-breadth, design D2: metacommands in comments are never ignored (qb64pe.bas 25426-25496)
'$INCLUDE: 'missing.bi'
REM $DYNAMIC
PRINT 1 '$STATIC
' $Format:Off
'$INCLUDEONCE
' plain comment with $INCLUDE:'x.bi' later in it
REM $FOO $DYNAMIC
'$INCLUDE 'no-colon.bi'
PRINT 2
