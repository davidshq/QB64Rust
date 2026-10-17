' TEST: ir
' The functions of QB64pe's file included after the last line (m3-network-embed design D7, D8; verification/v44_d_url):
' _ENCODEURL$ and _DECODEURL$ compiled from the file, with and without their $, as functions named without _IKW;
' the file's procedures are unchecked ($CHECKING:OFF). Without _SOCKETS_ the helper _WhatIsMyIP$ is not compiled.
$CONSOLE:ONLY
PRINT _ENCODEURL$("a b"); _DECODEURL("a%20b")
SYSTEM
