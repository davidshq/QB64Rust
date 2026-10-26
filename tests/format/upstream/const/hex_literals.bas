$Console:Only

Const test__byte%% = &H4F
Const test__byte_neg%% = &HFF

Const test__integer% = &H00FF
Const test__integer_neg% = &HFFF3

Const test__long& = &H00FF00FF
Const test__long_neg& = &HFFFFFF00 ' -256
Const test__long_neg16& = &HFF00 ' -256

Const test__ulong~& = &H4FFFFF00
Const test__ulong_neg~& = &HFFFFFF00
Const test__ulong_neg16~& = &HFF00 ' sign extension to &HFFFFFF00~&

Const test__int64&& = &H4FFFFFFFFFFFFF00
Const test__int64_neg&& = &HFFFFFFFFFFFFFF00
Const test__int64_neg32&& = &HFFFFFF00 ' sign extension to &HFFFFFFFFFFFFFF00
Const test__int64_neg16&& = &HFF00 ' sign extension to &HFFFFFFFFFFFFFF00

Const test__uint64_neg~&& = &HFFFFFFFFFFFFFF00
Const test__uint64_neg32~&& = &HFFFFFF00 ' sign extension to &HFFFFFFFFFFFFFF00
Const test__uint64_neg64~&& = &HFFFFFFFFFFFFFF00

Const test__uint64&& = &H4000000000000001

Const test__ulong_uinteger~& = &HFFFF~% ' should avoid sign extension due to unsigned type

Print "byte: "; test__byte%%; Hex$(test__byte%%) ' 4F
Print "byte negative: "; test__byte_neg%%; Hex$(test__byte_neg%%) ' FF

Print "integer: "; test__integer%; Hex$(test__integer%) ' FF
Print "integer negative: "; test__integer_neg%; Hex$(test__integer_neg%) ' FFF3

Print "long: "; test__long&; Hex$(test__long&) ' FF00FF
Print "long negative: "; test__long_neg&; Hex$(test__long_neg&) ' FFFFFF00
Print "long negative sign extension: "; test__long_neg16&; Hex$(test__long_neg16&) ' FFFFFF00

Print "ulong: "; test__ulong~&; Hex$(test__ulong~&) ' 4FFFFF00
Print "ulong negative: "; test__ulong_neg~&; Hex$(test__ulong_neg~&) ' FFFFFF00
Print "ulong negative sign extension: "; test__ulong_neg16~&; Hex$(test__ulong_neg16~&) ' FFFFFF00

Print "int64: "; test__int64&&; Hex$(test__int64&&) ' 4FFFFFFFFFFFFF00
Print "int64 negative: "; test__int64_neg&&; Hex$(test__int64_neg&&) ' FF00?
Print "int64 negative sign extension: "; test__int64_neg32&&; Hex$(test__int64_neg32&&) ' FF00?
Print "int64 negative sign extension: "; test__int64_neg16&&; Hex$(test__int64_neg16&&) ' FFFFFF00?

Print "uint64 negative: "; test__uint64_neg~&&; Hex$(test__uint64_neg~&&) ' FF00 - possible bug
Print "uint64 negative sign extension: "; test__uint64_neg32~&&; Hex$(test__uint64_neg32~&&) ' FF00 - possible bug
Print "uint64 negative sign extension: "; test__uint64_neg64~&&; Hex$(test__uint64_neg64~&&) ' FF00 - possible bug

Print "uint64: "; test__uint64&&; Hex$(test__uint64&&) ' 4000000000000001

Print "ulong uinteger: "; test__ulong_uinteger~&; Hex$(test__ulong_uinteger~&) ' 4000000000000001

System
