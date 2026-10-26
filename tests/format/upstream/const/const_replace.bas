$Console:Only

Const const__OR__test = 1 Or 2 ' 3
Const const__AND__test = 3 And 1 ' 1
Const const__XOR__test = 2 Xor 3 ' 1
Const const__mod__test = 20 Mod 3 ' 2

Const const__byte%% = 1%%
Const const__ubyte~%% = 2~%%
Const const__int% = 4%
Const const__uint~% = 8~%
Const const__long& = 16&
Const const__ulong~& = 32~&
Const const__int64&& = 64&&
Const const__uint64~&& = 128~&&
Const const__single! = 256!
Const const__double# = 512#
Const const__float## = 1024##

Const const__negative = -20000

' Test original casing, and UCASE
Const const_replace_test = const__OR__test + const__AND__test + const__XOR__test + const__mod__test
Const const_replace_test_ucase = const__OR__test + const__AND__test + const__XOR__test + const__mod__test

' Defined with suffix, used with suffix
Const const_replace_test_suffix = const__byte%% + const__ubyte~%% + const__int% + const__uint~% + const__long& + const__ulong~& + const__int64&& + const__uint64~&& + const__single! + const__double# + const__float##

' Defined with suffix, but missing when used
Const const_replace_test_no_suffix = const__byte + const__ubyte + const__int + const__uint + const__long + const__ulong + const__int64 + const__uint64 + const__single + const__double + const__float


Const const_replace_test_added_suffix = const__OR__test&& + const__AND__test&& + const__XOR__test&& + const__mod__test&&

Const const_replace_test_negative = 20 - const__negative

Print const_replace_test
Print const_replace_test_ucase
Print const_replace_test_suffix
Print const_replace_test_no_suffix
Print const_replace_test_added_suffix
Print const_replace_test_negative

System
