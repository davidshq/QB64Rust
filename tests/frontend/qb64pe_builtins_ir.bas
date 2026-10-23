' TEST: ir
$CONSOLE:ONLY
' _WRITEFILE in the IR (m4-qb64pe-sources task 2.1; verification\v52_c_files): two string slots, a variable-length
' and a fixed-length string
DIM f AS STRING, fx AS STRING * 4
_WRITEFILE f, "data"
_WRITEFILE fx + ".x", f + _READFILE$(f)
