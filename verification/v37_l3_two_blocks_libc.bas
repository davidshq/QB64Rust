$CONSOLE:ONLY
' DECLARE LIBRARY on Linux, one form, the twin of v37_x67 (m7-linux task 3.5): two DYNAMIC blocks of the C library
' (each declares the handle of one name).
DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"
    FUNCTION strlen~%& (s AS STRING)
END DECLARE
DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"
    FUNCTION abs& (BYVAL v AS LONG)
END DECLARE
PRINT strlen~%&("seven!!" + CHR$(0)); abs&(-3)
SYSTEM
