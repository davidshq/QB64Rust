' SYNTAX TEST "source.qb64rust" "numbers, suffixes, labels"
x = 42 + 3.5 + .5 + 1E10 + 2.5D-3 + 7#
'   ^^ constant.numeric.decimal.qb64rust
'        ^^^ constant.numeric.decimal.qb64rust
'              ^^ constant.numeric.decimal.qb64rust
'                   ^^^^ constant.numeric.decimal.qb64rust
'                          ^^^^^^ constant.numeric.decimal.qb64rust
'                                   ^^ constant.numeric.decimal.qb64rust
'                                    ^ storage.type.suffix.qb64rust
y = &HFF + &O17 + &B101 + &HFFFF~& + 5~%%
'   ^^^^ constant.numeric.radix.qb64rust
'          ^^^^ constant.numeric.radix.qb64rust
'                 ^^^^^ constant.numeric.radix.qb64rust
'                         ^^^^^^^^ constant.numeric.radix.qb64rust
'                               ^^ storage.type.suffix.qb64rust
'                                     ^^^ storage.type.suffix.qb64rust
z1& = count% + big&& + f! + d# + q## + b` + u~%
' <-- variable.other.qb64rust
' ^ storage.type.suffix.qb64rust
'          ^ storage.type.suffix.qb64rust
'                 ^^ storage.type.suffix.qb64rust
'                       ^ storage.type.suffix.qb64rust
'                            ^ storage.type.suffix.qb64rust
'                                 ^^ storage.type.suffix.qb64rust
'                                       ^ storage.type.suffix.qb64rust
'                                            ^^ storage.type.suffix.qb64rust
10 PRINT "line number"
' <-- entity.name.label.line-number.qb64rust
start:
' <----- entity.name.label.qb64rust
CLS: PRINT
' <--- support.function.qb64rust
GOTO start
' <---- keyword.control.qb64rust
