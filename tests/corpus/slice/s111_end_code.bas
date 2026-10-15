$CONSOLE:ONLY
' Slice (m3-asserts-debug-zlib task 4.1): END with an exit code: the prompt of END, then the code, -1 as an
' _INTEGER64 whose low 32 bits are the process's code (measured by hand, study\00 section 5: END -1 gives
' 0xFFFFFFFF).
DIM q AS _INTEGER64
q = -1
PRINT "x"
END q
