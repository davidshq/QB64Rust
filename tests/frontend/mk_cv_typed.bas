' TEST: typed
$CONSOLE:ONLY
' A number and its bytes (m3-mem task 5.1, design D5; verification\v32_c_mk_cv): MKI$ and MKL$ convert their number
' as a store into INTEGER and LONG would (a float to INTEGER rounded as a SINGLE, through qbr_float_to_long of the
' value as it is), MKS$ and MKD$ take it as it is; _MK$ and _CV keep the type named in their first slot (a 0 of the
' type; _BIT one bit wide); CVI ... CVD and _CV are typed as their type (a _BIT as an _INTEGER64).
DIM s AS STRING, f AS STRING * 4, i AS INTEGER, k AS LONG, d AS DOUBLE, sg AS SINGLE, q AS _INTEGER64
s = MKI$(i) + MKI$(k) + MKI$(1.7) + MKI$(d)
s = MKL$(k) + MKL$(d) + MKS$(sg) + MKS$(k) + MKD$(d) + MKD$(1)
s = _MK$(INTEGER, k) + _MK$(SINGLE, k) + _MK$(_UNSIGNED _BYTE, 255) + _MK$(_INTEGER64, d)
s = _MK$(_BIT, k) + _MK$(_UNSIGNED _BIT, k) + _MK$(_OFFSET, k) + _MK$(_FLOAT, d)
PRINT CVI(s); CVL(f); CVS(s); CVD(s + f)
PRINT _CV(_UNSIGNED LONG, s); _CV(_FLOAT, s); _CV(_BIT, s); _CV(_UNSIGNED _BIT, s); _CV(_UNSIGNED _OFFSET, s)
q = _CV(_INTEGER64, _MK$(_INTEGER64, -9)) + CVL(MKL$(-2))
SYSTEM
