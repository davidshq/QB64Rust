' TEST: cpp
' The C++ of QB64pe's file included after the last line (m3-network-embed design D7, D8): FUNC__ENCODEURL,
' FUNC__DECODEURL and, with _SOCKETS_ from _OPENCLIENT, FUNC__WHATISMYIP, the signatures libqb's network part calls;
' no event check (evnt) in the three procedures, as the old compiler writes them under $CHECKING:OFF.
$CONSOLE:ONLY
PRINT _ENCODEURL$("a b")
SYSTEM
SUB never
    PRINT _OPENCLIENT("x")
END SUB
