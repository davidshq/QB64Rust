$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x41_on_error_function_label)
PRINT "ok"
SYSTEM
abs: l = 1
ON ERROR GOTO abs
