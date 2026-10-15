$CONSOLE:ONLY
' Libraries without a header: a C .a (DECLARE LIBRARY and DECLARE STATIC LIBRARY), a C .o and a C++ .a.
' The prototypes come from the declarations; the old compiler's nm step decides C or C++ linkage.
DECLARE LIBRARY "./v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB c_inc (v AS LONG)
    FUNCTION c_len& (s AS STRING)
    FUNCTION c_half# (BYVAL v AS DOUBLE)
    FUNCTION c_hello$
END DECLARE
DECLARE STATIC LIBRARY "./v37s"
    FUNCTION c_halfs! (BYVAL v AS SINGLE)
    FUNCTION c_big&& (BYVAL v AS _INTEGER64)
END DECLARE
DECLARE LIBRARY "./v37o"
    SUB o_tick
    FUNCTION o_ticks&
END DECLARE
DECLARE LIBRARY "./v37cpp"
    FUNCTION cpp_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB cpp_inc (v AS LONG)
    FUNCTION cpp_half# (BYVAL v AS DOUBLE)
    SUB cpp_tick
    FUNCTION cpp_ticks&
END DECLARE

x& = 1
c_inc x&: cpp_inc x&
PRINT "c:"; c_add&(2, 3); x&; c_len&("abcd"); c_half#(5); c_hello$
PRINT "static:"; c_halfs!(3); c_big&&(4000000000)
o_tick: o_tick
PRINT "o:"; o_ticks&
PRINT "c++:"; cpp_add&(20, 22); cpp_half#(9)
cpp_tick: cpp_tick: PRINT "c++ ticks:"; cpp_ticks&
SYSTEM
