$CONSOLE:ONLY
' DECLARE DYNAMIC LIBRARY on Linux (m7-linux task 3.5): the C library's file name written whole. The old compiler's
' Linux build looks for liblibc.so.so.6, so it is not found.
DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/libc.so.6"
    FUNCTION strlen~%& (s AS STRING)
END DECLARE
PRINT strlen~%&("seven!!" + CHR$(0))
SYSTEM
