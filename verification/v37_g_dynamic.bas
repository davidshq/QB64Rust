$CONSOLE:ONLY
' DECLARE DYNAMIC LIBRARY: a DLL beside the program and one of System32, loaded when the program starts.
DECLARE DYNAMIC LIBRARY "./v37dll"
    FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB d_inc (v AS LONG)
    FUNCTION d_len& (s AS STRING)
    FUNCTION d_half# (BYVAL v AS DOUBLE)
    FUNCTION d_hello$
    SUB d_tick
    FUNCTION ticks& ALIAS d_ticks
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
END DECLARE

x& = 1: d_inc x&
d_tick: d_tick
PRINT "dll:"; d_add&(2, 3); x&; d_len&("abcd"); d_half#(5); d_hello$; ticks&
PRINT "kernel32:"; lstrlenA&("seven!!")
SYSTEM
