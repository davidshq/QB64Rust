$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7, review): a file that includes itself, guarded by $IF and $LET, so the
' second inclusion is inactive. Accepted (no cycle check) or rejected?
'$INCLUDE:'v16_inc/self_guarded.bi'
SYSTEM
