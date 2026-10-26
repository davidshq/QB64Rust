$Console:Only

Declare Library "./fastmath"
    Function Fast_Sqrt& (ByVal val As Long)
End Declare

value = Fast_Sqrt&(2000)
Print value;
System
