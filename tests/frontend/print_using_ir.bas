' TEST: ir
$CONSOLE:ONLY
' PRINT USING in the IR (m3-console-files task 4.1, design D1; verification\v30_a-v30_c, v30_x01-x19): a
' statement of its own with the items before USING, the format, the values and whether the line ends. A number is
' formatted as a SINGLE, DOUBLE, _FLOAT, _UNSIGNED _INTEGER64 or _INTEGER64 by what it is believed and stays as it
' is; TAB and SPC written as a value are blanks; a comma between values is a separator and nothing else
DIM b AS _BYTE, i AS INTEGER, l AS LONG, ul AS _UNSIGNED LONG, q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64
DIM s AS SINGLE, d AS DOUBLE, f AS _FLOAT, o AS _OFFSET, uo AS _UNSIGNED _OFFSET, v AS STRING, x AS STRING * 4
' Each value type
PRINT USING "##"; b; i; l; ul; q; uq; o; uo
PRINT USING "#.##^^^^"; s; d; f; 1234!; 1234#; 5; i + 1; l / 2
PRINT USING "& ! \  \"; v; x; "lit" + v
' The format any string expression, evaluated once
PRINT USING v; i
PRINT USING v + LEFT$("###xx", 3); i
PRINT USING fmt$; 1; 2; 3
' TAB and SPC among the values, and as part of a string
PRINT USING "## ##"; 1; TAB(10); 2; SPC(i); 3
PRINT USING "&"; TAB(3) + "x"
' Items before USING, with and without a separator, a zone
PRINT "a"; USING "##"; 5
PRINT 1; 2 USING "##"; 3
PRINT "a", USING "##"; 5
' The end of the line: a trailing separator of either kind; a comma between values
PRINT USING "##"; 1;
PRINT USING "##"; 1,
PRINT USING "##x"; 1, 2, 3
' To a file, with items before and a trailing semicolon; a question mark
PRINT #1, USING "##.##"; d
PRINT #l + 1, "x"; USING v; i; v;
? USING "##"; 5
' LPRINT: the printer as the destination, with and without USING (the program's runtime parts name it)
IF i THEN LPRINT "a"; i, v
IF i THEN LPRINT USING "##"; i;
SYSTEM
FUNCTION fmt$
    fmt$ = "#"
END FUNCTION
