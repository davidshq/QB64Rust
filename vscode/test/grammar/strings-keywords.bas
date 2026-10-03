' SYNTAX TEST "source.qb64rust" "strings, keywords, case"
PRINT "it's": x = 1
'     ^^^^^^ string.quoted.double.qb64rust
'           ^^^^^^^^ - comment
'             ^ - string
print 1
' <----- support.function.qb64rust
Print 1
' <----- support.function.qb64rust
PRINT 1
' <----- support.function.qb64rust
a$ = CHR$(65) + left$(b$, 2)
' <- variable.other.qb64rust
'^ storage.type.suffix.qb64rust
'    ^^^^ support.function.qb64rust
'               ^^^^^ support.function.qb64rust
FOR i = 1 TO 3: NEXT
' <--- keyword.control.qb64rust
'         ^^ keyword.control.qb64rust
'               ^^^^ keyword.control.qb64rust
DIM n AS _UNSIGNED LONG
' <--- storage.modifier.qb64rust
'        ^^^^^^^^^ storage.type.qb64rust
'                  ^^^^ storage.type.qb64rust
IF a AND NOT b THEN c = _TRUE ELSE c = _FALSE
'    ^^^ keyword.operator.word.qb64rust
'        ^^^ keyword.operator.word.qb64rust
'                       ^^^^^ constant.language.qb64rust
'                                      ^^^^^^ constant.language.qb64rust
COLOR BrightWhite
'     ^^^^^^^^^^^ support.constant.color.qb64rust
x = "unterminated
'   ^^^^^^^^^^^^^ string.quoted.double.qb64rust
printer = 1
' <------- - support.function
