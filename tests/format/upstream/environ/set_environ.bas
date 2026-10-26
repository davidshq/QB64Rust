$Console:Only
On Error GoTo ehandler

'Test setting with =
Environ "FOO=BAR"
Print Environ$("FOO")


'Test settings with space
Environ "VAR VAL"
Print Environ$("VAR")


'Test setting value with spaces with = separator
Environ "ABC=DEF GHI"
Print Environ$("ABC")


'Test setting value with spaces with space separator
Environ "JKL MNO PQR"
Print Environ$("JKL")


'Test overwriting existing
Environ "X=XY"
Environ "X=ZZZ"
Print Environ$("X")


'Test unset variable with = separator
Environ "NAME=LUKE"
Environ "NAME="
Print "["; Environ$("NAME"); "]"


'Test unset variable with space separator
Environ "TEXT BOO"
Environ "TEXT "
Print "["; Environ$("TEXT"); "]"

'Test no separator
Environ "NOSEP"

System

ehandler:
Print "Error"; Err; "line"; _ErrorLine
Resume Next
