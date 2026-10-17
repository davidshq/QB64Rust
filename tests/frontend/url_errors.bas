' TEST: check-fail
' The names of QB64pe's file included after the last line a program cannot use or declare (m3-network-embed design
' D7; verification/v44_x47-x50): the helper _WhatIsMyIP$, with or without a network function, a SUB named like it,
' a FUNCTION named like _EncodeURL$. Each is an error, none "not supported yet"; the messages are new.
$CONSOLE:ONLY
IF 0 THEN PRINT _WHATISMYIP$
IF 0 THEN PRINT _WHATISMYIP$: PRINT _OPENCLIENT("x")
PRINT _ENCODEURL$("a b")
SYSTEM
SUB _WhatIsMyIP
END SUB
FUNCTION _EncodeURL$ (u$)
    _EncodeURL$ = u$
END FUNCTION
