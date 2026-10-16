$CONSOLE:ONLY
' DECLARE LIBRARY, one form: two DYNAMIC blocks of one DLL (each declares the handle DLL_kernel32).
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION GetTickCount&
END DECLARE
PRINT lstrlenA&("seven!!"); GetTickCount& > 0
SYSTEM
