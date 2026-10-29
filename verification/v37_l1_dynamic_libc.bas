$CONSOLE:ONLY
' DECLARE DYNAMIC LIBRARY on Linux, the twin of v37_g_dynamic (m7-linux task 3.5): a shared object beside the
' program and the C library, loaded when the program starts. On Windows the C library's name is not found.
DECLARE DYNAMIC LIBRARY "./v37dll"
    FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB d_inc (v AS LONG)
    FUNCTION d_len& (s AS STRING)
    FUNCTION d_half# (BYVAL v AS DOUBLE)
    FUNCTION d_hello$
    SUB d_tick
    FUNCTION ticks& ALIAS d_ticks
END DECLARE
DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"
    FUNCTION strlen~%& (s AS STRING)
END DECLARE

x& = 1: d_inc x&
d_tick: d_tick
PRINT "dll:"; d_add&(2, 3); x&; d_len&("abcd"); d_half#(5); d_hello$; ticks&
PRINT "libc:"; strlen~%&("seven!!" + CHR$(0))
SYSTEM
