$CONSOLE:ONLY
' Verification (D-065): a file that starts with $INCLUDEONCE, included a second time by a name in other
' letters. On Windows it is the same file. The reference QB64pe compares the names as written and includes it
' twice; QB64pe since 37641e597 compares them without case on Windows, as QB64Rust does, and includes it once.
'$INCLUDE:'v70_inc/once_case.bm'
'$INCLUDE:'v70_inc/ONCE_CASE.BM'
PRINT "end"
SYSTEM
