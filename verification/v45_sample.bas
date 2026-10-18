$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.3): the sample behind the certification of the CONST evaluator's
' functions. Each function the evaluator computes with a library that is not correctly rounded is called here at
' run time with a _FLOAT argument, which compiles to the C++ qb64pe.exe's own evaluator runs (std::cos on long
' double, func_log, func_arcsec, pow2, ...), at 500 arguments each. A line is the function's number, the argument's
' number and the 10 bytes of the _FLOAT result in hex (low byte first). The arguments are exact binary fractions:
' h = (i * 2654435761) AND &HFFFFF, u = h / 2^20 in [0, 1); crates\sema\src\certify.rs reproduces them.
DEFINT A-Z
DIM i AS LONG, h AS _INTEGER64, h2 AS _INTEGER64, u AS _FLOAT, v AS _FLOAT, x AS _FLOAT, r AS _FLOAT
DIM f AS INTEGER, n AS LONG
n = 500
FOR f = 1 TO 19
    FOR i = 1 TO n
        h = (i * 2654435761&&) AND &HFFFFF&&
        h2 = (i * 40503&& + 12345&&) AND &HFFFF&&
        u = h / 1048576##
        v = h2 / 65536##
        SELECT CASE f
            CASE 1: x = u * 16## - 8##: r = SIN(x)
            CASE 2: x = u * 16## - 8##: r = COS(x)
            CASE 3: x = u * 3## - 1.5##: r = TAN(x)
            CASE 4: x = u * 32## - 16##: r = ATN(x)
            CASE 5: x = u * 2## - 1##: r = _ACOS(x)
            CASE 6: x = u * 2## - 1##: r = _ASIN(x)
            CASE 7: x = u * 32## - 16##: r = EXP(x)
            CASE 8: x = u * 16## + 0.0009765625##: r = LOG(x)
            CASE 9: x = u * 16## + 0.0009765625##: r = x ^ (v * 8## - 4##)
            CASE 10: x = u * 16## + 1##: r = _ARCSEC(x)
            CASE 11: x = u * 16## + 1##: r = _ARCCSC(x)
            CASE 12: x = u * 32## - 16##: r = _ARCCOT(x)
            CASE 13: x = u * 16## - 8##: r = _SECH(x)
            CASE 14: x = u * 16## + 0.0009765625##: r = _CSCH(x)
            CASE 15: x = u * 16## + 0.0009765625##: r = _COTH(x)
            CASE 16: x = u * 3## - 1.5##: r = _SEC(x)
            CASE 17: x = u * 3## + 0.0009765625##: r = _CSC(x)
            CASE 18: x = u * 3## + 0.0009765625##: r = _COT(x)
            CASE 19: x = u * 16## + 0.0009765625##: r = SQR(x)
        END SELECT
        PRINT LTRIM$(STR$(f)); ":"; LTRIM$(STR$(i)); " "; hex10$(r)
    NEXT
NEXT
SYSTEM

FUNCTION hex10$ (r AS _FLOAT)
    DIM s AS STRING, t AS STRING, k AS INTEGER
    s = _MK$(_FLOAT, r)
    FOR k = 1 TO 10
        t = t + RIGHT$("0" + HEX$(ASC(s, k)), 2)
    NEXT
    hex10$ = t
END FUNCTION
