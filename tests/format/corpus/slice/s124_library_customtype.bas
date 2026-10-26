$Console:Only
' DECLARE CUSTOMTYPE LIBRARY with a header (v37h.h in this program's .files folder): the function is
' reached through a pointer of the declared type, so the declaration's types may differ from the C
' function's (verification\v37_h).
Declare CustomType Library "v37h"
    Function h_add& (ByVal a As Long, ByVal b As Long)
    Function addll&& Alias h_add (ByVal a As Long, ByVal b As Long)
    Sub h_pl (v As Long)
End Declare

x& = 4: h_pl x&
Print "customtype:"; h_add&(1, 2); addll&&(3, 4); x&
System
