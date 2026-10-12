' TEST: cpp
$CONSOLE:ONLY
' DATA, READ and RESTORE as C++ (m2-builtin-statements task 4.4, design D5 and D6; the old compiler's lines are in
' study\00 section 5): the data in global.txt as inline_data[], each item's bytes and a comma, a quoted item in its
' quotes, with data_size and one data_at_LABEL_<NAME> byte offset per label a RESTORE names; READ: per target
' func_read_float(data,&data_offset,data_size,<type code>) stored by the rule of its place, func_read_int64 and
' func_read_uint64 for the 64-bit integers, sub_read_string for a string, with no pending-error test between the
' targets; RESTORE: data_offset=0 or data_offset=data_at_LABEL_<NAME>
TYPE rec
    n AS LONG
    f AS STRING * 4
END TYPE
DIM b AS _BYTE, ui AS _UNSIGNED INTEGER, l AS LONG, q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64
DIM sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT, o AS _OFFSET, b3 AS _BIT * 3, ub3 AS _UNSIGNED _BIT * 3
DIM s AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING, r AS rec
before:
DATA 1, "two" ,  three  x,,"un
lab2: DATA "x"
READ b, ui, l, q, uq
READ sg, db, fl, o, b3, ub3
READ s, fx
READ a(l), sa(l), r.n, r.f
RESTORE
RESTORE before
RESTORE lab2
RESTORE after
p
SYSTEM
after:

SUB p
    DATA 5
    READ zz
    RESTORE lab2
END SUB
