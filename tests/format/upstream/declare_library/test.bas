$Console:Only

Declare Library
    Function isdigit& (ByVal c As Long)
End Declare

value = isdigit&(Asc("2"))
Print value;
System
