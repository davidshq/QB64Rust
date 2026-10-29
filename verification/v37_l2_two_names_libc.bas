$CONSOLE:ONLY
' DECLARE LIBRARY on Linux, one form, the twin of v37_x06 (m7-linux task 3.5): two DLL names, the second the C
' library.
DECLARE DYNAMIC LIBRARY "./v37dll", "/usr/lib/x86_64-linux-gnu/c:6"
    FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
SYSTEM
