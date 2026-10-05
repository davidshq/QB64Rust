$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7): a nested include that exists only in the main file's folder (not in the
' including file's folder). run.sh compiles from verification\, so if the path as written were taken relative
' to the working directory, the file would be found.
'$INCLUDE:'v16_inc/x2.bi'
SYSTEM
