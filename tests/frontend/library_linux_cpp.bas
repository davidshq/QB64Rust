' TEST: cpp-linux
$CONSOLE:ONLY
' The C++ of DECLARE LIBRARY on Linux (m7-linux task 3.4, design D3; qb64pe.bas 5243-5249, 5470-5476, 5887-5894
' with os$ = "LNX"). The test takes the libraries as library_cpp.bas does, but a DYNAMIC library as the shared
' object "./lib<name>.so" (frontend.rs, fixed_linux_libraries). A DYNAMIC block's handle is a void *, loaded with
' dlopen(..., RTLD_LAZY), its function pointers typed without CALLBACK and found with dlsym, a failure told by
' dlerror(); with $ERRORLOCATION:ON each records its line first. The other forms are as on Windows.
$ERRORLOCATION:ON
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
DECLARE LIBRARY "./v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE DYNAMIC LIBRARY "./v37s"
    FUNCTION s_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB s_inc (v AS LONG)
    FUNCTION s_hello$
END DECLARE
DECLARE DYNAMIC LIBRARY "/usr/lib/x86_64-linux-gnu/c:6"
    FUNCTION strlen~%& (s AS STRING)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "./v37c"
    FUNCTION ct_add& ALIAS c_twice (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE

DIM x AS LONG
x = h_l&(2) + c_add&(2, 3) + s_add&(2, 3) + strlen~%&("seven!!" + CHR$(0)) + ct_add&(5, 6)
s_inc x
PRINT s_hello$
