$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.1): literals with the suffixes of the new numeric types, in and
' out of range; with a unary minus; in an addition (the type the old compiler gives the literal); and HEX$ of
' a negative literal (its width shows the believed type, study\00 section 5, v20_e_radix).
ON ERROR GOTO h
PRINT "%%:"; 127%%; 128%%; 200%%; 255%%; 256%%; -128%%; -129%%
PRINT "~%%:"; 255~%%; 256~%%; 300~%%; -1~%%; -255~%%
PRINT "~%:"; 65535~%; 65536~%; -1~%
PRINT "~&:"; 4294967295~&; 4294967296~&; -1~&
PRINT "~&&:"; 18446744073709551615~&&; -1~&&
PRINT "&& past max:"; 9223372036854775808&&
PRINT "% past max:"; 32768%; -32769%
PRINT "& past max:"; 2147483648&; -2147483649&
PRINT "&H:"; &HFF~%%; &HFF%%; &H1FF~%%; &HFFFF~%; &HFFFF%; &HFFFFFFFF~&; &HFFFFFFFFFFFFFFFF~&&; &HFFFFFFFFFFFFFFFF&&
PRINT "&H + 0:"; &HFF%% + 0; &HFFFF% + 0; &HFFFFFFFF~& + 0; &H1FF~%% + 0
PRINT "&B &O:"; &B111~%%; &B11111111%%; &O777~%; &O177777%
PRINT "bit:"; 1`; 0`; 3`2; 2`2; 1~`; 2~`; 5~`3; 9~`3; 9`3
PRINT "+ 0:"; 200%% + 0; 255~%% + 0; 300~%% + 0; -1~%% + 0; -1~% + 0; -1~& + 0; -1~&& + 0; 3`2 + 0; 5~`3 + 0
PRINT "+ 1:"; 255~%% + 1; 65535~% + 1; 4294967295~& + 1; 18446744073709551615~&& + 1; 127%% + 1
PRINT "* 2:"; 255~%% * 2; 65535~% * 2; 4294967295~& * 2; 18446744073709551615~&& * 2
PRINT "HEX$ -1:"; HEX$(-1%%); " "; HEX$(-1~%%); " "; HEX$(-1~%); " "; HEX$(-1~&); " "; HEX$(-1~&&); " "; HEX$(-1`); " "; HEX$(-1`5)
PRINT "HEX$ -2:"; HEX$(-2%%); " "; HEX$(-2~%%); " "; HEX$(-2~%); " "; HEX$(-2~&); " "; HEX$(-2~&&)
PRINT "compare:"; -1~& < 0; -1~& > 0; 255~%% = -1%%; -1~&& > 0
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
