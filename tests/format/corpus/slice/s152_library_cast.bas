$Console:Only
' Slice (m4-builtins-param-types, design D2): _CAST as a BYVAL argument of DECLARE LIBRARY keeps its cast, to a
' parameter of a wider type and to one of another kind (v37h.h in this program's .files folder, and the C runtime
' with no header); the overload a header's C++ name takes for a cast argument (h_over: 1 for int32_t, 2 for double).
Declare Library "v37h"
    Function h_l& (ByVal v As Long)
    Function h_d# (ByVal v As Double)
    Function h_ll&& (ByVal v As _Integer64)
    Function over& Alias h_over (ByVal v As Double)
End Declare
Declare Library
    Function labs& (ByVal v As Long)
End Declare
Dim k As Long, sg As Single, d As Double, q As _Integer64
k = 300: sg = -6.75: d = 7.5: q = 4294967297
Print h_l&(_Cast(_Byte, k)); h_l&(_Cast(_Unsigned _Byte, k)); h_l&(_Cast(Integer, k * 300))
Print h_d#(_Cast(Long, sg)); h_d#(_Cast(Integer, d)); h_ll&&(_Cast(Long, q))
Print labs(_Cast(_Byte, k)); labs(_Cast(Long, sg))
Print over&(_Cast(Long, d)); over&(_Cast(Integer, sg)); over&(k); over&(d)
System
