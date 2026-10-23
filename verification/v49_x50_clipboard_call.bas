$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): CALL _CLIPBOARD$ with an argument.
CALL _CLIPBOARD$("abc")
PRINT _CLIPBOARD$
_CLIPBOARD$ = ""
SYSTEM
