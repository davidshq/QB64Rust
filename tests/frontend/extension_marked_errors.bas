' TEST: check-fail
DIM a(1 TO 2) AS STRING
a(1) = "x"
s (a(1))
SUB s (t AS STRING)
END SUB
' A form qb64rust marks "not supported yet" and QB64pe builds: the program of the extension's tests of "Build with
' QB64pe" (vscode\test\integration\suite\buildQb64rust.test.ts), which read this file. When a change builds the form,
' this snapshot loses its "not supported yet": put another marked form at line 4 and its message in that test. This
' comment stays below the program so that editing it moves no line.
