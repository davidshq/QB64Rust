' TEST: check-fail
$CONSOLE:ONLY
' Forms the old compiler accepts and m3-get-put-bload leaves "not supported yet", one a line (design D4, D5;
' SOMEDAY.md, "Left marked by m3-get-put-bload"): in the array's place of graphics GET and PUT an index that calls a
' FUNCTION and an array of _BIT (the other places compile since m4-memory-records, get_put_kinds_cpp.bas). Every
' diagnostic below is a mark, none an error. VARPTR and VARSEG of the main module's variables compile since
' m4-memory-records (varptr_kinds_typed.bas), of a procedure's variables and parameters since qbasic-testcases
' (varptr_procedure_ok.bas). The file form of GET and PUT is compiled since m3-console-files
' (file_get_put_ir.bas). A statement with a template is read after CALL since m3-sound (call_template_ir.bas)
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM i AS INTEGER, l AS LONG, t AS STRING, fx AS STRING * 8, one AS rec
DIM a(100) AS INTEGER, r(10) AS rec, ts(10) AS STRING, fs(10) AS STRING * 4, bt(10) AS _BIT
GET (1, 2)-(3, 4), a(f%(1))
PUT (1, 2), bt
SYSTEM

FUNCTION f% (n AS INTEGER)
    f% = n
END FUNCTION
