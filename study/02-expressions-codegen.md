# QB64pe study 02 — Expression evaluation, built-in registration, C++ code generation

Scope: `source\qb64pe.bas` (28,828 lines), `source\subs_functions\`, `source\utilities\{const_eval,type,elements}.bas`, `source\utilities\arrcpy.bm`, `internal\c\qbx.cpp`, and the runtime headers the emitted code touches.
All `file:line` references are to the tree at `..\QB64pe` (HEAD `16f629784e`, `Version$ = "4.7.0-GLFW"`, `source\global\version.bas:13`). Unqualified line numbers mean `source\qb64pe.bas`.

> **Tree caveat.** This tree contains machinery that, to my knowledge, is not in stock QB64-PE: arrays as TYPE members (`udtearrayelements`, `_Static`/`_Dynamic` member storage, `id.dynudt`), whole-array assignment `a() = b()`, `_ARRAYCOPY`, and a coordinate-preserving REDIM mode (`redimoption = 3`, called "_Retain" in comments). It is interleaved with the classic paths in `evaluate`, `udtreference`, `refer`, `setrefer`, `evaluatetotyp`, `allocarray`, `dim2`. I describe the classic behaviour and mark the extra layer as **[member-array layer]**. I did not diff against upstream, so treat "not in stock" as unverified.

Element-list conventions used throughout (`source\global\constants.bas:6`):

| Name | Char | Role |
|---|---|---|
| `sp` | CHR$(13) | separates elements of a tokenised line / expression |
| `sp2` | CHR$(10) | "no-space" joiner in layout (pretty-print) strings |
| `sp3` | CHR$(26) | field separator inside encoded *references* |
| CHR$(241) | | unary minus marker inserted by `fixoperationorder` |

Element helpers live in `source\utilities\elements.bas` (`getelement$`:2, `getelements$`:92, `insertelements`:139, `numelements`:170, `removeelements`:184).

---

## 1. From element list to C++ text

### 1.1 Pipeline

```
lineformat$ (24588)        source text -> sp-separated elements; numeric literals normalised
fixoperationorder$ (23502) precedence -> explicit bracketing; literals get type suffixes;
                           CONSTs substituted; names case-corrected; builds tlayout$
evaluate$ (19355)          bracketed element list -> C++ text + type code (or a *reference*)
evaluatefunc$ (20214)      one function call (built-in or user) -> C++ call text
evaluatetotyp$ (22141)     evaluate + coerce to a requested target type (or a pseudo-type -2..-8)
refer$ (25647)             reference -> rvalue C++ text (method 0) or C variable name (method 1)
setrefer (26815)           reference + rhs -> emits an assignment statement
```

Callers almost always do `e$ = fixoperationorder$(e$)` then `evaluatetotyp(e$, targettype)` (e.g. MID$ statement 8490-8507). `evaluatetotyp` itself no longer calls `fixoperationorder` (22142); `setrefer` does (26819) unless `method = 1`.

Everything is string-to-string. There is no AST; the "IR" is the bracketed element list, and the output is C++ source text concatenated bottom-up. Side effects during evaluation are real: `evaluate` can auto-create variables/arrays (emitting declarations into the data buffers), set `stringprocessinghappened`, `arrayprocessinghappened`, `constequation = 0`, and `recompile = 1`.

### 1.2 Numeric literal normalisation (lineformat, 24588-25557)

`lineformat$` rewrites each literal to a canonical element, optionally followed by `,originaltext` (kept only for layout; `fixoperationorder` strips it at 23913-23917).

| Source form | Canonical element | Type decided by |
|---|---|---|
| integer, no suffix `123` | `123` | suffix added later in `fixoperationorder` (see 1.3 H) |
| integer with suffix `5&`, `5~%%`, `` 5`3 `` | `5&` + `,5&` | explicit suffix (24701-24763). `%&`/`~%&` rejected: "Cannot use _OFFSET symbols after numbers" |
| float, no suffix `1.5`, `.1` | `1.5E+0` / `0.1E+0` | significant digits: <=7 and in range -> SINGLE (`E`); <=16 -> DOUBLE (`D`); else _FLOAT (`F`) (24765-24819) |
| `1E5` / `1D5` / `1F5` | `1.0E+5` / `1.0D+5` / `1.0F+5` | exponent letter |
| `1.5!` / `1.5#` / `1.5##` | `E` / `D` / `F` forms | suffix |
| `&HFFFF` | `-` `sp` `1%,&HFFFF` | value, not digit count: <=4 hex digits -> `%`, <=8 -> `&`, else `&&`; **signed** wrap as in QB (24913-24967). So `&HFFFF` = -1, `&H8000` = -32768, `&HFFFFFFFF` = -1&. With an explicit `~` suffix no wrap. >16 digits -> "Overflow" |
| `&O…`, `&B…` | same scheme (24988 ff.) | |

Note the hex negative form is emitted as two elements (`-`, number) and `fixoperationorder` rule C deliberately skips re-negating `&`-origin literals (23645).

String literals arrive as `"text",len` elements with `\\` and `\ooo` (octal) escapes; `fixoperationorder` unescapes for layout only (23882-23907); `evaluate` emits `qbs_new_txt_len("text",len)` (19794-19800).

### 1.3 fixoperationorder (23502-24473)

Recursive (`fixoperationorder_rec`, depth counter `fooindwel`). Returns the rewritten element list; sets global `tlayout$` to the pretty-printed layout of the same expression.

Top-level-only steps (when `fooindwel = 1`):

| Step | Lines | What |
|---|---|---|
| dup-op check | 23518-23530 | `AND AND`, `OR OR`, `XOR XOR`, `IMP IMP`, `EQV EQV`, `_ANDALSO _ANDALSO`, `_ORELSE _ORELSE` -> error |
| A | 23532-23556 | bracket balance: "Missing (" / "Missing )" |
| B | 23558-23600 | sign cleanup: `+ +` -> `+`; `- +` -> `-`; `op - -` -> `op` (double negation removed) |
| C | 23604-23669 | negation: a `-` at start, after `(`, `,` or any operator is unary. If followed by a number (and that number is not followed by `^`) fold into a negative literal `-5`; otherwise replace `-` by CHR$(241) |

Every level:

| Step | Lines | What |
|---|---|---|
| D | 23677-23715 | `^` followed by CHR$(241): wrap `{ neg ... }` up to the next operator that is not `^`, negation or `NOT`. So `2 ^ -3 * 4` = `(2 ^ (-3)) * 4`, and `2 ^ -x ^ 2` = `2 ^ (-(x ^ 2))` |
| E | 23718-23735 | find lowest (`lco`) and highest (`hco`) operator level at bracket depth 0 |
| F | 23737-23798 | if more than one level present, wrap every maximal run between `lco`-level operators in `{ }`. Recursion (step I) then handles the next level inside each `{ }` |
| NOT exception | 23743-23763 | if `lco` is NOT: rewrite `a NOT b...` as `a { NOT { b... } }` and rescan |
| G | 23800-23817 | remove the temporary power-negation braces that are now nested |
| unary chains | 23822-23828 | `neg neg x` and `NOT NOT x` get nested braces |
| H | 23830-24348 | classify every depth-0 element: string, number, operator, constant, variable/array/UDT/function. Adds integer suffixes, substitutes CONST values, corrects name case in layout |
| I | 24354-24465 | recurse into each `(…)`/`{…}` group, splitting at depth-1 commas; `{ }` become `( )` in the output |

**Precedence table** (`isoperator`, 24518-24554; return value = level, higher binds tighter):

| Level | Operators | Notes |
|---|---|---|
| 1 | `_ORELSE` | QB64pe extension, short-circuit |
| 2 | `_ANDALSO` | QB64pe extension, short-circuit |
| 3 | `IMP` | |
| 4 | `EQV` | |
| 5 | `XOR` | |
| 6 | `OR` | |
| 7 | `AND` | |
| 8 | `_NEGATE` | logical not (unary) |
| 9 | `NOT` | bitwise not (unary); hard-coded `lco = 9` at 23743 |
| 10 | `=  >  <  <>  <=  >=` | hard-coded `isop = 10` at 20035 |
| 11 | `+  -` | binary |
| 12 | `MOD` | |
| 13 | `\` | |
| 14 | `*  /` | |
| 15 | CHR$(241) | unary minus |
| 16 | `^` | |

Consequences: `-2 ^ 2 = -4`; `2 ^ 3 ^ 2 = 64` (left-assoc; all same-level operators are folded left to right by `evaluate`); `NOT a = b` is `NOT (a = b)`; `a MOD b \ c` is `a MOD (b \ c)`; `a \ b * c` is `a \ (b * c)`. All match QB4.5.

**Step H details**

- Integer literal without suffix gets the smallest of `%`, `&`, `&&`, `~&&` that holds it, by string-length/lexical compare (23929-23944). Negative literals: `%` down to -32768, `&` down to -2147483648 … (the `&` test is `f3$ < "-2147483648" AND LEN = 11`, i.e. the boundary value -2147483648 itself falls through to `&&`; looks accidental).
- CONST substitution (23969-24113): hash lookup `HASHFLAG_CONSTANT`, scope = current sub or global, must be `constdefined`. **A non-array variable of the same name visible in scope overrides the constant** (the "STATIC variable overriding" check 24000-24023). Numeric constants are re-rendered as literal text plus type suffix (`typevalue2symbol$`), floats as `mantissaE/D/F±exp`; a suffix written on the constant name (`MYCONST&`) retypes it with no range check ("todo: range checking" 24053). String constants are replaced by the stored quoted-string element.
- Names: four lookup attempts (`try_method` 1-4): local scope without / with the DEFtype-implied suffix, then any scope without / with it (24123-24141). Functions and arrays followed by `(` have their argument list skipped here and formatted later by recursion. UDT member chains `a.b.c` are walked against `udtename()` so each member name is validated and case-corrected ("Element not defined", 24288).
- `bare_arrays` flag: the argument list of `UBOUND(`/`LBOUND(` is recursed with `bare_arrays = TRUE` so `UBOUND(arr)` resolves `arr` to the array rather than a same-named scalar (24145-24150, 24404-24409).

### 1.4 evaluate (19355-20209)

Input must be single-precedence-level per bracket group (guaranteed by 1.3). Algorithm:

1. **Block building** (19385-19681). Scan elements at depth 0; each produces a *block* with `evaledblock` state 0 (raw token/operator), 1 (C++ value text) or 2 (reference), and a `blocktype`.
   - name + `(` where id is an array -> `arrayreference` -> reference `id␚index` (19430-19487); if array of UDT and followed by `.`, continue into `udtreference` with a byte offset `(index)*elemsize` (19449-19472).
   - simple variable -> `makeidrefer`: block = id number as text, type = `id.t + ISREFERENCE` (19493-19501, 25558).
   - UDT variable -> `udtreference` (19505-19537).
   - function (`id.subfunc = 1`) -> count top-level commas, call `evaluatefunc(args$, argcount, typ)` (19543-19577). A function referenced without `(` is called with `""`,0.
   - unknown name followed by `(` -> **auto-create an array** via `dim2(name, type, 1, "10[,10…]")` then `GOTO reevaluate` (19584-19646). Error under `OPTION _EXPLICIT`/`_EXPLICITARRAY`. Inside a SUB the creation code goes to the data file (`autoarray = 1`) and a `STATIC name()` pre-declaration is honoured via `staticarraylist`.
   - `( … )` group -> recursive `evaluate`; result wrapped in parentheses unless it is a pointer type (19664-19679).
2. **Literal / implicit variable pass** (19697-19867) for state-0 non-operator blocks:
   - number: SINGLE if it contains `E`, DOUBLE if `D`, _FLOAT if `F`. `D`/`F` are replaced by `E`; `F` gets an `L` suffix. **SINGLE literals are emitted without an `f` suffix, i.e. as C++ `double` constants** (19721-19725). Integers: type from suffix; >32-bit get `ll`/`ull` (19727-19733). Literal text is padded with spaces.
   - quoted string -> `qbs_new_txt_len(…)`/`qbs_new_txt(…)`, type `ISSTRING`.
   - valid identifier -> **auto-create a scalar** with `dim2(x$, typ$, 1, "")` using suffix or DEFtype letter (19806-19837); error under `OPTION _EXPLICIT`.
3. **Reference short-circuit** (19870-19882): if the whole expression is one block that is a reference, return the reference unresolved (`typ` has `ISREFERENCE`). This is how callers get lvalues, by-reference args, `VARPTR` operands etc. Otherwise every reference block is turned into a value with `refer(…, 0)`.
4. **Operator fold** (19892-20185), left to right. `typ` starts as the type of the first operand. For each operator: `operatorusage` (25563) gives a usage code, operand class masks and result class; then conversion, type markup and text generation as below.
5. Join blocks -> return.

#### operatorusage table (25563-25645)

`lhs`/`rhs` masks: 1 = integral ok, 2 = float ok, 4 = string. `result`: 1 integer, 2 float, 4 string, 8 boolean, 0 = "use markup".

| Operator | Emitted C++ | lhs | rhs | result |
|---|---|---|---|---|
| string `+` | `qbs_add(a,b)` | 4 | 4 | string |
| string `= <> > < >= <=` | `qbs_equal`, `qbs_notequal`, `qbs_greaterthan`, `qbs_lessthan`, `qbs_greaterorequal`, `qbs_lessorequal` `(a,b)` (runtime returns -1/0) | 4 | 4 | bool |
| `^` | `pow2(a,b)` | 1+2 | 1+2 | float |
| unary minus | `-(a)` | — | 1+2 | markup |
| `/` | `a/ b` | float lhs: 2 / else 1+2 | float lhs: 1+2 / else **2** | float |
| `* + -` | `a*b` `a+b` `a-b` | 1+2 | 1+2 | markup |
| `= > < <> <= >=` | `-(a==b)` etc. | 1+2 | 1+2 | bool |
| `MOD` | `qb_safe_mod(a,b)` | 1 | 1 | int |
| `\` | `qb_safe_idiv(a,b)` | 1 | 1 | int |
| `IMP` | `~a|b` | 1 | 1 | int |
| `EQV` | `~a^b` | 1 | 1 | int |
| `XOR` `OR` `AND` | `a^b` `a|b` `a&b` | 1 | 1 | int |
| `_ORELSE` `_ANDALSO` | `-(a||b)` `-(a&&b)` | 1 | 1 | int |
| `NOT` | `~(a)` | — | 1 | int |
| `_NEGATE` | `-(!(a))` | — | 1 | int |

`qb_safe_idiv`/`qb_safe_mod` are C++ templates written at the top of `global.txt` (1697-1698): `error(11)` and result 0 on zero divisor, otherwise plain C `/` and `%`.

#### Conversion rules applied per operator (19990-20032)

- operand is float but the operator only accepts integers -> wrap in **`qbr(x)`**, type becomes int64.
- operand is integer but the operator requires float (only the rhs of `/` when the lhs is not float) -> wrap in **`((long double)(x))`**, type becomes _FLOAT.
- string where number required / number where string required -> "Cannot convert string to number" / "Cannot convert number to string".
- **Float comparison narrowing** (20035-20057): when *both* sides of a comparison are floating, both are cast to the *smaller* float type: `((float)(a)) == ((float)(b))` if either is SINGLE, else `(double)` if either is DOUBLE. This is the fix for `S! = 2.1 : IF S = 2.1`. Mixed int/float comparisons are left to C++ promotion.

#### Result type "markup" (20061-20134)

The type code is the compiler's *belief* about the result; **no cast is emitted** for `+ - *`, so the actual C++ arithmetic follows C++ usual arithmetic conversions on the operand C types.

| Case | Type code assigned |
|---|---|
| either operand float (non-string) | float of the larger float width among the float operands (an integer operand does not widen it) |
| both integer | signed 64-bit (`64&`); unsigned 64-bit only when **both** are unsigned 64-bit |
| result = int (`MOD`, `\`, bitwise, logical) | keep integer markup; a float/string markup becomes int64 |
| result = float (`/`, `^`) | if markup is not float -> _FLOAT (`ISFLOAT+256`) |
| `^` | QB-like: each integer operand maps <=16 bit -> SINGLE, 32 bit -> DOUBLE, 64 bit -> _FLOAT; float operands keep width; result = max. Value is still `long double` from `pow2` |
| comparisons | `32&` (LONG), value 0 or -1 |
| string `+` | `ISSTRING` |

Practical consequences a rewrite must reproduce:

- `INTEGER + INTEGER`, `LONG * LONG` etc. are computed in C `int` (32-bit) even though the compiler labels them int64; mixing in an `_INTEGER64` operand gives real 64-bit arithmetic. There is **no overflow error** (QB4.5 raised error 6); signed overflow is C++ UB that wraps in practice. The source comment admits it: "THIS IS THE IDEAL MARKUP FOR A 64-BIT SYSTEM / In reality 32-bit C++ only marks-up to 32-bit integers" (20076-20077).
- `int / int` is evaluated as `a/ ((long double)(b))` -> 80-bit result typed _FLOAT. `PRINT 1 / 3` therefore prints more digits than QB4.5's SINGLE. *(Verified in `09`:
it prints 16 digits, ` .3333333333333333`, i.e. DOUBLE-style formatting, not the ~19 digits of a `_FLOAT`.)* `single / int` stays SINGLE-typed, `int / single` too (no cast on either side).
- A SINGLE literal is a C++ double constant, so `d# = 0.1` stores the double 0.1, not QB4.5's widened single.
- `pow2` (`internal\c\libqb\include\qbmath.h:59`): `error(5)` for negative base with non-integer exponent, else `std::pow` in long double.

#### _OFFSET rules (19950-19988, 20099-20110, 20175)

If either operand is `_OFFSET`: for `+ - \ MOD` and bitwise ops both sides are forced integral; for `*` with a float operand, and always for `/`, both sides are cast to `long double` and the result is wrapped in `qbr(…)`; `^` is an error ("Operator '^' cannot be used with an _OFFSET"). The result type is forced to `_OFFSET` (signed if any offset operand is signed, else unsigned) except for comparisons.

#### Emission (20136-20168)

| usage | text |
|---|---|
| 1 | `lhs OP rhs` |
| 2 | `fn(lhs,rhs)` |
| 3 | `-(lhs OP rhs)` — C++ bool 0/1 negated to QB 0/-1 |
| 4 | `~lhs OP rhs` |
| 5 | `OP(rhs)` |
| 6 | `-(OP(rhs))` |

There is no extra parenthesisation between same-level operators; correctness relies on each operand already being an atom or a parenthesised group.

### 1.5 Rounding and implicit numeric conversion

Float -> integer conversion happens in three places with the same rule (`evaluatetotyp` 23240-23249, built-in/user argument passing 21881-21890 and 12249-12258):

| Target integer width | Wrapper | Implementation (`internal\c\libqb\include\rounding.h`) |
|---|---|---|
| <=16 bits | `qbr_float_to_long(x)` | x87 `flds; fistpl` (or `nearbyintf`) -> int32 |
| 17..31 bits (only `_BIT*n`) | `qbr_double_to_long(x)` | `fldl; fistpl` (or `nearbyint`) |
| >=32 bits | `qbr(x)` | `fldt; fistpll` on long double -> int64, with a special path for values above INT64_MAX (returns the uint64 bit pattern) |

All are **round-half-to-even** (banker's rounding, FPU default mode; `fpu_reinit` sets control word 0x37F). No range check: after rounding, the C assignment truncates to the destination width silently (`x% = 70000` wraps; QB4.5 would raise Overflow).

Note the first wrapper narrows the *expression* to `float` first, so a DOUBLE expression assigned to an INTEGER is rounded from its single-precision value ("**32 rounding fix" comment). Integer -> float and integer -> integer conversions are plain C assignment conversions; integer -> narrower integer truncates.

Explicit conversion functions are special-cased in `evaluatefunc` (see section 5.3): `CINT`/`CLNG` (range-checked, error 6), `INT` (`std::floor`, type unchanged), `FIX` (`func_fix_double/float`), `_ROUND` (`qbr`, int64, unchecked), `CSNG`, `CDBL`, `_CAST`.

### 1.6 String vs numeric

- Type code bit `ISSTRING` decides; mixing raises compile errors ("Cannot convert number to string", "Illegal string-number conversion" 23210).
- All string expressions are `qbs*`. Literals and function results are *temporary* qbs objects registered in `qbs_tmp_list`; any statement that set `stringprocessinghappened` is followed by `qbs_cleanup(qbs_tmp_base,0);` (e.g. 12407, 26926). When a string-derived value is used as a condition the value is threaded through cleanup: `qbs_cleanup(qbs_tmp_base, expr)` (6295, 6673, 6725). `qbs_cleanup` is a template returning its second argument (`internal\c\libqb\include\qbs.h:97`).
- `STRING$(n, "x")` with a string second arg is rewritten to `(e->chr[0])` (21294-21303).

### 1.7 _BIT, _UNSIGNED, _FLOAT

- `_BIT*n` scalars are stored in `int32`/`int64` (unsigned variants `uint32`/`uint64`) and always live in conventional memory (18211-18222). Reading is a plain dereference. Writing (setrefer 27055-27083): unsigned -> `*v=(e)&mask;`; signed -> assign then sign-extend by testing bit n-1 (`|= bitmaskinv` / `&= bitmask`).
- `_BIT` arrays are bit-packed: read `getbits(n,(uint8*)(arr[0]),idx)` / `getubits`; write `setbits(n,(uint8*)(arr[0]),tmp_long,val)` (25746-25754, 26979-26992; `internal\c\libqb\include\bitops.h`). Arrays are limited to 63 bits (18147); storage is `elements*bits/8+1` bytes (15084). `_BIT` cannot be a UDT member reference ("Cannot resolve bit-length variables inside user defined types"), cannot be passed by array-element reference, SWAPped, or `_OFFSET`ed.
- `_UNSIGNED` only changes the C type of storage and the type-code flag. In expression markup, unsignedness survives only for uint64 op uint64 (1.4). Passing a signed variable to an unsigned parameter of the same width (or vice versa) passes the **same storage** with a pointer cast (12180-12184, 21784-21788).
- `_FLOAT` is `long double`, stored in 32 bytes (`mem_static_malloc(32)`, 18899-18903), type size field 256. `MKL$/CVL`-style helpers use `f`.
- `_OFFSET` is `ptrszint`/`uptrszint`, flagged `ISOFFSET`; see offset rules above. `_OFFSET(x)` -> `((uptrszint)(<address expr>))` (20937-20948).

---

## 2. References and assignment

### 2.1 Reference encoding

A reference is a *string* plus a type code carrying `ISREFERENCE`. Three shapes:

| Kind | Text | Type code | Built by |
|---|---|---|---|
| scalar variable | `<idnumber>` | `id.t + ISREFERENCE` | `makeidrefer` 25558 |
| array element | `<idnumber>␚<flat index C expr>` | `id.arraytype + ISARRAY + ISREFERENCE` | `arrayreference` 15563 |
| whole array `a()` | `<idnumber>␚0` | same | 15584-15587 |
| UDT (whole or member) | `<idnumber>␚<udt#>␚<element# or 0>␚<byte offset C expr>` | `udtetype(element) + ISUDT + ISREFERENCE` (+`ISARRAY` when the root is an array, +`ISINCONVENTIONALMEMORY`); whole UDT: `udt# + ISUDT + ISREFERENCE` | `udtreference` 19129 |
| **[member-array layer]** whole member array | `<id>␚<udt#>␚<element#>␚<offset>` with type `ISARRAY` and no `ISUDT` | | 19246-19262, gated by global `udt_allow_bare_array` |

`␚` = `sp3`. `udt#` is the UDT of the innermost container of the element, not of the variable ("udt of the element, not of the id", 19131-19132). The byte offset is a C expression string such as `((0+4)+8)` — nested member offsets are accumulated numerically in bits (`o`) per level and appended as `(prev+bytes)` (19336); bit-misaligned members error out ("Non-byte aligned user defined type").

Array flat index (15609-15644), first dimension varies fastest:

```
array_check((i1)-A[4*(n-1)+4], A[4*(n-1)+5])
 + array_check((i2)-A[4*(n-2)+4], A[4*(n-2)+5]) * A[4*(n-2)+6] + ...
```

With `$CHECKING:OFF` the `array_check` calls become plain subtraction (15629-15637). `array_check` is `inline ptrszint array_check(uptrszint index, uptrszint limit)` in `qbx.cpp:474`. Index expressions are evaluated with `evaluatetotyp(…, 64&)`, so float indexes are `qbr`-rounded. Wrong number of indexes: "Cannot change the number of elements an array has!".

### 2.2 refer$ (25647-25832): reference -> C++

`method = 0` yields an rvalue/lvalue expression, `method = 1` the C identifier (pointer or descriptor).

| Reference | method 0 | method 1 |
|---|---|---|
| numeric scalar | `*__LONG_X` | `__LONG_X` |
| string scalar | `__STRING_X` (a `qbs*`) | same |
| fixed string scalar | `__STRING10_X` | same |
| bit scalar | `*__BIT3_X` / `*__UBIT3_X` | without `*` |
| numeric array elem | `((int32*)(__ARRAY_LONG_X[0]))[idx]` | `__ARRAY_LONG_X` |
| string array elem | `(((qbs**)(__ARRAY_STRING_X[0]))[idx])` | descriptor name |
| fixed-string array elem | `qbs_new_fixed(&((uint8*)(A[0]))[(idx)*N],N,1)` (a temp qbs aliasing the bytes) | descriptor name |
| bit array elem | `getbits(n,(uint8*)(A[0]),idx)` | descriptor name |
| UDT numeric member | `*(int32*)(((char*)__UDT_X)+(off))` | `__UDT_X` / `__ARRAY_UDT_X` |
| UDT var-len string member | `*((qbs**)((char*)__UDT_X+(off)))` | |
| UDT fixed string member | `qbs_new_fixed(((uint8*)__UDT_X)+(off),N,1)` | |
| whole UDT, method 0 | error "User defined types in expressions are invalid" (25686) | |

For arrays of UDT the base is `__ARRAY_UDT_X[0]`.

### 2.3 Assignment (`assign` 17343, `setrefer` 26815)

`assign` splits at the first depth-0 `=`, runs `fixoperationorder` + `evaluate` on the lhs, requires `ISREFERENCE` ("Expected variable =, look for conflict with a CONST name", 17428), then `setrefer lhsref, typ, rhs, 0`. A one-element lhs is first looked up as a *local* non-UDT variable so that a FUNCTION's own name resolves to its return variable rather than recursing (17404-17419).

Emitted forms (setrefer):

| lhs | Emitted C++ |
|---|---|
| numeric scalar | `*__LONG_X=<evaluatetotyp(rhs, type)>;` (27099-27104) |
| string scalar | `qbs_set(__STRING_X,<rhs>);` then `qbs_cleanup(qbs_tmp_base,0);` (27038-27052) |
| fixed string scalar | same `qbs_set` — the runtime honours `qbs->fixed` (pad/truncate) |
| bit scalar | see 1.7 |
| numeric array element | `tmp_long=<idx>;` `if (!is_error_pending()) ((int32*)(A[0]))[tmp_long]=<rhs>;` (27016-27024) |
| string array element | `tmp_long=<idx>;` `if (!is_error_pending()) qbs_set( ((qbs**)(A[0]))[tmp_long],<rhs>);` + cleanup |
| fixed string array elem | `tmp_long=<idx>;` `if (!is_error_pending()) qbs_set(qbs_new_fixed(&((uint8*)(A[0]))[tmp_long*N],N,1),<rhs>);` |
| bit array element | `tmp_long=<idx>;` `if (!is_error_pending()) setbits(n,(uint8*)(A[0]),tmp_long,<rhs>);` |
| UDT numeric member | `*(int16*)(((char*)__UDT_X)+(off))=<rhs>;` (26928-26936) — index of an array-of-UDT root is inside `off`, **not** hoisted to `tmp_long` and not guarded by `is_error_pending()` |
| UDT string member | `qbs_set(<member qbs expr>,<rhs>);` + cleanup |
| whole UDT = UDT | `copy_full_udt dst, src` (26905): memcpy of fixed part and `qbs_set` per variable-length string member. Both sides must have the same `udt#` and element 0 ("Expected = similar user defined type"). rhs may also be a non-reference UDT pointer value (e.g. `_MEM` function results: `((char*)&e)` / `((char*)e)`, 26869-26877) |
| `_MEM` member | error "Cannot set read-only element of _MEM TYPE" (`u = 1`, 26849) |

The index-then-guard pattern means: the index (with its bounds check) is evaluated **before** the rhs, and the store is skipped if the index raised an error.

Order-of-evaluation quirk: for scalars the rhs is evaluated by C++ in a single expression statement; no temporaries.

**[member-array layer]**: `assign` detects `x() = y()` (`GetAsgRefSyntax`, `GetWholeAsgRef`, `EmitAssignWhole`, 17384-17400) and emits a whole-array copy; `asg_guard_on` routes rhs evaluation through `EvalAsgRHS`/`CheckAsgRHS` (16809-16850) purely to produce better diagnostics for bare/whole array operands.

### 2.4 Statements that write through references (all special-cased in the main loop)

| Statement | Lines | Emitted |
|---|---|---|
| `MID$(v$, start[, len]) = e$` | 8447-8525 | `sub_mid(<v qbs>,<start>,<len or 0>,<e>,<1 if len given else 0>);` target must be a string reference |
| `ASC(v$[, pos]) = n` | 8341-8441 | `tqbs=<v>; if (!is_error_pending()){ tmp_long=<n>; if (!is_error_pending()){ if (tqbs->len){tqbs->chr[0]=tmp_long;}else{error(5);} }}` (with position: `tmp_fileno=<pos>` and a range test) |
| `LSET` / `RSET v$ = e$` | 11326-11374 | `sub_lset(<v>,<e>);` / `sub_rset(<v>,<e>);` |
| `SWAP a, b` | 11376-11505 | strings: `swap_string(a,b);` whole UDTs: `swap_8/16/32/64(src,dst)` when the UDT is exactly 1/2/4/8 bytes, else `swap_block(src,dst,bytes)`; numerics: `swap_<bits>(&a,&b)` with `swap_longdouble` for _FLOAT. Types must match after stripping pointer/cmem/array/unsigned/UDT flags ("Type mismatch"); `_BIT` not allowed. Note **SWAP of UDTs containing variable-length strings just swaps the bytes (i.e. the qbs pointers)** |
| `_MEMGET m, offs, v` | 10170-10260 | uses `evaluatetotyp(v, -5)` (size) and `-6` (address); sized 1/2/4/8 -> typed load, else `memmove`; with checking on, a block validating `mem_block` bounds and lock id -> errors 300/308/309 |
| `_MEMPUT`, `_MEMFILL` | 10267, 10406 | similar |

---

## 3. Storage model in generated code

### 3.1 C identifier naming

`scope$` (26206): `module$ + "__"` for SHARED ids, else `module$ + "_" + subfunc$ + "_"`. `module$` is empty for the main module, `subfunc$` is `""` in main code, `SUB_NAME` or `FUNC_NAME` inside procedures. Names are upper-cased.

| BASIC | C identifier (main / inside `SUB Foo`) | C declaration |
|---|---|---|
| `x%` | `__INTEGER_X` / `_SUB_FOO_INTEGER_X` | `int16 *__INTEGER_X=NULL;` |
| `x~%` | `__UINTEGER_X` | `uint16 *` |
| `x%%`, `x~%%` | `__BYTE_X`, `__UBYTE_X` | `int8 *`, `uint8 *` |
| `x&`, `x~&` | `__LONG_X`, `__ULONG_X` | `int32 *`, `uint32 *` |
| `x&&`, `x~&&` | `__INTEGER64_X`, `__UINTEGER64_X` | `int64 *`, `uint64 *` |
| `x!`, `x#`, `x##` | `__SINGLE_X`, `__DOUBLE_X`, `__FLOAT_X` | `float *`, `double *`, `long double *` |
| `x%&`, `x~%&` | `__OFFSET_X`, `__UOFFSET_X` | `ptrszint *`, `uptrszint *` |
| ``x`n``, ``x~`n`` | `__BITn_X`, `__UBITn_X` | `int32 *`/`int64 *` (n>32), unsigned variants |
| `x$` | `__STRING_X` | `qbs *` |
| `x AS STRING * 10` | `__STRING10_X` | `qbs *` (fixed) |
| `x AS mytype` | `__UDT_X` | `void *` |
| any array | `__ARRAY_<TYPE>_X` (e.g. `__ARRAY_LONG_X`, `__ARRAY_STRING10_X`, `__ARRAY_UDT_X`) | `ptrszint *` (descriptor) |
| by-value temp for a call | `passN` | `int32 passN;` |
| function-result _MEM temp | `funcN` | `mem_block funcN;` (22084-22089) |

Because the type is part of the C name, `x%`, `x&` and `x$` are distinct variables, exactly as in QB. `validname`/`autoIncForceUScore` deal with names that would collide with C/C++ identifiers.

### 3.2 Allocation of scalars (dim2, 17624-18939)

Every variable is a **heap/arena pointer**, never a C local. Two texts are produced per variable:

- a declaration written to `defdatahandle` — `global.txt` for main-module variables, `dataN.txt` for procedure locals (N = procedure number);
- an initialiser written to `DataTxtBuf` (`maindata.txt` or `dataN.txt`), guarded so that it runs once per pointer lifetime:

```c
int32 *__LONG_X=NULL;                       // declaration
if(__LONG_X==NULL){                         // initialiser
__LONG_X=(int32*)mem_static_malloc(4);
*__LONG_X=0;
}
```

Variants:

| Case | Initialiser |
|---|---|
| in conventional memory (`cmemlist(id)` set) | `cmem_sp-=4; __LONG_X=(int32*)(dblock+cmem_sp); if (cmem_sp<qbs_cmem_sp) error(257);` |
| var-len string | `if (!__STRING_X)__STRING_X=qbs_new(0,0);` (or `qbs_new_cmem(0,0)`); free list gets `qbs_free(__STRING_X);` (18102-18115) |
| fixed string | `__STRING10_X=qbs_new_fixed((uint8*)mem_static_malloc(10),10,0); memset(__STRING10_X->chr,0,10);` — initial content is NUL bytes, not spaces (17998-18019) |
| UDT | `__UDT_X=(void*)mem_static_malloc(bytes); memset(__UDT_X,0,bytes);` plus `initialise_udt_varstrings` / `free_udt_varstrings` for variable-length string members (17822-17855) |
| _FLOAT | 32 bytes reserved (18899-18903) |
| `_BIT*n` scalar | always cmem: `cmem_sp-=4;` regardless of n — for n>32 the C type is `int64` but only 4 bytes are reserved (18141, 18215). Looks like a latent bug |

Scopes:

| BASIC scope | Where declared | Notes |
|---|---|---|
| main-module variable | `global.txt` + `maindata.txt` | C globals; name prefix `__` |
| `DIM SHARED` / `COMMON SHARED` | same; `id.share` set so `scope$` yields `__` from inside procedures | |
| procedure local | `dataN.txt` (declaration **and** initialiser), included inside the C function body | so the pointer is a C local re-created per call; storage comes from the `mem_static` arena and is released by resetting the arena pointer at exit (see the control-flow section); strings/arrays are freed via `freeN.txt` |
| `STATIC` local, or all locals of a `SUB … STATIC` | declaration redirected to `global.txt`, initialiser to `maindata.txt`, frees to `mainfree.txt` (17655-17661, 18930-18935), but the C name keeps the procedure scope prefix (`_SUB_FOO_LONG_X`) | initialised once at program start |
| parameters | no storage: the C parameter *is* the pointer; `dim2` is called with `dimsfarray = 1` -> "Creates an ID only (no C++ code)" (17628-17633) | `id.sfid`/`id.sfarg` link the id to its procedure and position |

The id table records `musthave` (suffix required to refer to it: implicitly/suffix-declared variables) versus `mayhave` (suffix optional: `DIM x AS LONG`) — `findid` 23352-23389. `regid` (25834) enforces QB name-clash rules (reserved words usable only with `$`, variables vs. user SUB/FUNCTION names, same-scope duplicates, CONST clashes).

### 3.3 Conventional memory emulation and the recompile loop

`cmem` is the emulated 1 MB DOS memory; `dblock` is the DGROUP-like block inside it; `cmem_sp` is a downward stack pointer for scalars, `cmem_static_pointer`/`cmem_dynamic_base`/`cmem_dynamic_malloc` serve arrays. A variable is placed there only if something needs a 16-bit address for it:

- `VARPTR`, `VARPTR$`, `VARSEG`, `SADD` on an id not yet flagged: the function emits the placeholder `[CONVENTIONAL_MEMORY_REQUIRED]`, sets `cmemlist(id) = 1` and `recompile = 1` (21313-21319, 21334-21340, 21404-21410, 21516-21522).
- passing a variable/array to a procedure parameter whose `sfcmemargs(target)` byte is 1 (the callee uses such a function on its parameter) does the same (21674-21680, 21793-21798, 12081-12086).

`recompile = 1` restarts the entire compilation with the accumulated `cmemlist`; ids must therefore be numbered identically across passes. The same restart mechanism resolves the number of dimensions of array parameters (`sflistn`/`sfelelist`, 2927-2934, and `nelereq` 21691-21698). A rewrite can replace this with a pre-pass, but the *observable* results must match: `VARPTR` results are offsets from `&cmem[1280]` (`((unsigned short)(((uint8*)p)-&cmem[1280]))`), `VARSEG` of DGROUP data is the literal `80`, array `VARSEG` is `(ptr - &cmem[0]) / 16`, and procedure arguments use `varptr_dblock_check`/`varseg_dblock_check` because the caller's variable may not be in `dblock` (21463-21506, 21523-21559).

### 3.4 Arrays (allocarray, 14897-15561)

Descriptor: `ptrszint *A = mem_static_malloc((4*dims+4+1)*ptrsz)` (15090-15096).

| Slot | Content |
|---|---|
| `[0]` | data pointer (`nothingvalue`, or `&nothingstring` for string arrays, while undefined) |
| `[1]` | reserved ("could be used to store a bit offset") |
| `[2]` | flags: 1 = defined/initialised, 2 = static, 4 = in cmem, 8 = descriptor-layout UDT **[member-array layer]** |
| `[3]` | not written by `allocarray`; (only the [member-array layer] reads `desc[3]` as a dimension count for member descriptors) |
| `[4+4k .. 7+4k]` | per dimension, **stored in reverse order** (last dimension at `[4]`, first at `[4*(dims-1)+4]`): lower bound, element count, multiplier, reserved |
| `[4*dims+4]` | `mem_lock*` for `_MEM` validity tracking (type 4) |

Multiplier of the first dimension is 1; each later dimension's multiplier = previous multiplier × previous count (15022-15027), i.e. column-major, first index fastest. An undefined dynamic array has lower bound 2147483647 and count 0 so any access fails `array_check` (15512-15517).

Bounds: `DIM a(10)` uses `OPTION BASE` as the lower bound (special case: a literal `0` upper bound forces `0 TO 0`, 14970-14980). Counts `<=0` -> `error(5)`; constant reversed bounds are compile errors (15007-15015).

**Static vs dynamic** (15056-15068): static iff all bounds are compile-time constant (`constequation` stays 1 through `evaluatetotyp`) **and** not inside a procedure (unless under STATIC), not `$DYNAMIC`, not `REDIM`, not `STATIC a(n)`. Static arrays are allocated once in the data section (`mem_static_malloc` or the cmem static area, zero-filled; strings each `qbs_new(0,0)`), flags `1+2` (+4). Dynamic arrays emit inline code into `main.txt` at the DIM statement:

```c
if (A[2]&2){ error(10); }else{            // static array cannot be redefined
  if (A[2]&1){ if (!error_occurred) error(10); }else{   // DIM (not REDIM) of defined array
    <build alloc_new_desc[], overflow checks -> error(257)>
    A[0]=(ptrszint)calloc((size_t)alloc_req_bytes,1);   // or cmem_dynamic_malloc + memset
    if (!A[0]) error(257);
    A[2]|=1;
    <commit descriptor>
}}
```

and, once per array, cleanup code into the free buffer (15393-15409, 15475-15500): free strings, `free`/`cmem_dynamic_free` the block, `free_mem_lock`.

**REDIM** (`redimoption` 1) skips the "already defined" error; the old block is captured first and released after the new one is allocated ("Do not destroy the old allocation until the new one has succeeded", 15217-15252), and the `_MEM` lock id is bumped so stale `_MEM` blocks become invalid (15221).
**REDIM _PRESERVE** (`redimoption` 2): copies `min(old_total, new_total)` elements **linearly** (strings via `qbs_set`, raw via `AppendRawRedimPreserve` 14815) — data is preserved by flat position, so only resizing the last dimension keeps coordinates. `redimoption = 3` preserves by coordinates (`AppendRetainCoordinateWalk` 14782) **[fork]**.
Changing the number of dimensions is a compile error; changing between `STRING` and `STRING * n` is "Type mismatch" (17941, 18046).

Arrays first referenced without DIM are auto-dimensioned `0..10` (well, `OPTION BASE..10`) per dimension (19607).

Array parameters: `dim2(…, "?")` creates an id with `arrayelements = -1` (unknown) linked to the procedure argument; the count is learned at the first use or call ("lucky guess" 15603, 14941) and may force a recompile.

`variablesize$` (22101) yields the byte size expression: constant for scalars, `STRING_X->len` for var-len strings, `bytes*(A[2]&1)*A[5]*A[9]…` for arrays.

### 3.5 Strings

`qbs` (`internal\c\libqb\include\qbs.h:14`): `chr`, `len`, `in_cmem`, `cmem_descriptor`, `cmem_descriptor_offset`, `listi`, `tmp`, `tmplisti`, `fixed`, `readonly`, `field`.

- Variables own one `qbs*` for their lifetime; assignment is `qbs_set(dst, src)` (copy; honours `fixed`).
- Temporaries (`qbs_new_txt`, function results, `qbs_add`) are tracked in `qbs_tmp_list`; `qbs_cleanup(qbs_tmp_base, v)` frees all temps above the base index. Each procedure saves/sets `qbs_tmp_base` on entry so callee cleanup does not free the caller's pending temps.
- `tqbs`, `tmp_long`, `tmp_fileno` are shared scratch globals used by emitted statements (see 2.3/2.4).
- Fixed-length strings inside arrays/UDTs are raw bytes; every access wraps them in `qbs_new_fixed(ptr, n, 1)` (a *temporary* qbs header that aliases the bytes), which is why such accesses set `stringprocessinghappened`.
- Variable-length strings inside UDTs are `qbs*` slots in the UDT block (pointer-sized), initialised/freed by `initialise_udt_varstrings` / `free_udt_varstrings`, and deep-copied on UDT assignment.

---

## 4. Calling convention

### 4.1 Common argument logic

The same algorithm exists twice: for functions in `evaluatefunc` (21603-21939) and for SUB statements in the main loop (11646-12313). For each argument, `targettyp = CVL(MID$(id.arg, i*4-3, 4))`:

| Target type | Source | What is passed |
|---|---|---|
| has `ISPOINTER`, numeric, source is a reference of the *same* base type (compare on size, bit, UDT, float, string flags only) | variable | the variable's pointer `__LONG_X` (true by-reference) |
| | array element | `(&(((int32*)(A[0]))[idx]))` |
| | UDT member passed to scalar param | `(int32*)(void*)( ((char*)(__UDT_X)) + (off) )` |
| | whole UDT to UDT param | `(void*)( ((char*)(__UDT_X)) + (off) )` |
| | same but sign differs | same pointer with a C cast `(uint16*)` — still aliases |
| has `ISPOINTER`, numeric, source is an expression, a different type, or wrapped in parentheses `(x)` (`dereference = 1`, 12017/20351) | | **by-value temp**: `int32 passN;` is declared in the data file and `&(passN=<converted expr>)` is passed. If the callee needs cmem: `int32 *passN=NULL;` + cmem allocation and `&(*passN=expr)`. **There is no copy-back.** |
| has `ISPOINTER + ISARRAY` | must be `name()` (reference `id␚0`) of identical element type (and fixed-string length) | the descriptor pointer `__ARRAY_LONG_X`. Errors: "Expected arrayname()", "Incorrect array type passed to sub/function", "Passing arrays with a differing number of elements to a SUB/FUNCTION is not supported" |
| string (pointer or not) | any string | the `qbs*` — a variable's own qbs (by reference) or a temporary |
| no `ISPOINTER` (built-ins, `BYVAL`) | any numeric | value, float->int rounded with `qbr*` as in 1.5 |
| `-1` | any numeric | value cast explicitly to its own C type (`(int16)(…)`, `(long double)(…)`; bits -> `(int64)`), "any numeric type" built-ins such as `STR$`, `ABS` |
| `-2` | variable/array | `byte_element((uint64)addr, bytes_to_end, &byte_element_N)` — address plus remaining size |
| `-3` | numeric array name, `()` optional | as `-2`, auto-appending `()` (11765-11854) |
| `-4` | variable or element (GET/PUT) | `byte_element(addr, element_bytes, …)`; var-len strings divert GET/PUT to `sub_get2`/`sub_put2`; omitted variable diverts to `field_get`/`field_put` (11622-11638, 11867-11956) |
| `-5`, `-6` | | size expression only / address expression only (`LEN`, `_OFFSET`, `_MEMGET`) |
| `-7`, `-8` | | five comma-separated values `(ptrszint)addr, size, typecode, elementsize, lock` for `_MEM(var)` / `_MEMELEMENT` — spliced into the call so the C function receives 5 parameters |

Type mismatches: string<->number are compile errors with ordinal messages ("2nd sub argument requires a string"); a UDT param requires the same UDT.

After each SUB call: `qbs_cleanup(qbs_tmp_base,0);` if strings were involved (12407).

**User FUNCTION/SUB differences from built-ins:** user procedures register every non-BYVAL parameter with `ISPOINTER` (so everything is by reference with by-value temps on mismatch), built-ins mostly take values. A whole array passed to a scalar parameter of a user procedure is rejected ("Whole array cannot be passed to a scalar SUB parameter", 11995-12007).

### 4.2 DECLARE LIBRARY calls (`id.ccall = 1`)

Registration: 2862-3060 (pre-pass) — `BYVAL` strips `ISPOINTER` from the parameter type (2943-2945; BYVAL is rejected outside DECLARE LIBRARY, strings cannot be BYVAL 5706); the C call name is the `ALIAS` or the BASIC name; a function's `callname` is prefixed with a marker cast `(  ctype  )name` (2998).

At the call site (12296-12309, 21943-21956, 22091-22095):

- a string argument is passed as `(char*)(<qbs expr>)->chr` — no NUL termination is added; callers must append `CHR$(0)`;
- an argument whose text is exactly `0` becomes `NULL`;
- a nested library-function result has its marker cast removed (`removecast$`);
- a function declared as returning a string is wrapped: `qbs_new_txt((  char*  )fn(...))`;
- non-BYVAL numeric args are pointers exactly as for BASIC procedures (including `passN` temps).

Prototype/loader emission (static, DYNAMIC, CUSTOMTYPE) is covered in section 8.

### 4.3 Optional arguments: the `specialformat` mini-language

`id.specialformat` is a template for the argument syntax of a built-in. Two independent interpreters exist.

#### Grammar

```
format   := item*
item     := '?'                      an expression argument
          | '[' format ']'           optional group (nestable)
          | '{' alt ('|' alt)* '}'   keyword choice; alt = one or more space-separated words
          | <any other char>         literal punctuation that must appear: , ( ) # = - ; etc.
```

If `specialformat` is empty and `id.args = n`, the format `?,?,…,?` is synthesised (26239-26243).

#### Statement interpreter: `seperateargs(a$, ca$, pass&)` (26211-26813)

Phase 1 (26253-26341) flattens the format into `lastt` items with: `T` (0 = `?`, 1 = literal or single-alternative keyword, n = n-way keyword choice), `Opt()` alternatives, `OptWords()` word counts, `Lev` (bracket depth), `EntryLev` (depth of the nearest enclosing group that already has an item before this one — 0 if this item opens its group), `DitchLev`.

Phase 2 (26358-26494) decides what is passed to C:

- literals are never passed (`DontPass = 1`);
- a single-alternative `{WORD}` at depth 0 is mandatory syntax and not passed;
- a `?` is always passed (the expression, or `NULL` when omitted);
- a multi-alternative `{A|B|C}` is passed as the 1-based index of the alternative found (or `NULL`/0 when omitted);
- each optional group needs a way for the runtime to know whether it was present:
  - if the group contains a multi-alternative choice, that argument's non-zero value is the indicator and no flag is spent (`PassRule = -index`);
  - otherwise the group gets one bit in the trailing `passed` bitmask, allocated in order of scanning: all depth-1 groups left to right (1, 2, 4, …), then depth-2 groups, etc.; a group that contains only a single-alternative keyword (e.g. `[{STEP}]`) also gets a bit and the keyword itself is not passed.

Phase 3 (26511-26749) matches the actual elements against the items with **backtracking**: an optional group is first assumed present; a `?` consumes elements up to the nearest occurrence (at bracket depth 0, not crossing a depth-0 comma) of the next literal/keyword; on failure the most recent taken branch is flipped and the scan restarts from that point. Exhausting all branches gives "Syntax error - Reference: <hr_syntax>".

Phase 4 (26765-26806) compacts the passed items into `separgs(1..id.args)`: an expression's element list, `"N-LL"` for an omitted argument, or the alternative index. `pass&` receives the bitmask; the function result says whether a bitmask argument is needed at all. `separgslayout()` carries keyword text for the code formatter.

The generic SUB path (11561-12414) then evaluates each `separgs` per 4.1, emits `NULL` for `N-LL`, and appends `,<passed>` when needed:

```c
sub_locate(NULL,10,NULL,NULL,NULL,2);        // LOCATE , 10     format "[?][,[?][,[?][,[?][,?]]]]"
```

Examples (formats quoted from the registration table — see section 5 for the verbatim list):

| Format | Meaning |
|---|---|
| `?,?` | two mandatory expressions |
| `[?]` | one optional expression; passes `NULL,0` or `expr,1` |
| `[?],[?]` | either may be omitted but the comma is mandatory |
| `[?][,[?][,?]]` | trailing-optional chain with nested groups |
| `[{STEP}](?,?)[,[?]…]` | optional keyword consumes a flag bit, parentheses and commas are literals |
| `{ON|OFF|STOP}` | mandatory keyword choice, passed as 1/2/3 |
| `[#]?,?` | optional literal `#` |

#### Function interpreter (evaluatefunc 20254-20305, 22052-22063; `isValidArgSet` elements.bas:334)

Functions do **not** use `seperateargs`; only `?`, `[`, `]` (and commas) are meaningful.

- `hasFunctionElement` (elements.bas:258) reports for each comma-separated position whether an expression is present;
- `isValidArgSet` checks that within each bracket group arguments are all present or all absent, and an inner group present implies its outer group present;
- omitted arguments are emitted as `NULL` (20332-20339); "Last function argument cannot be empty";
- a trailing bitmask is **always** appended when a format exists: `,0|1|2|…` with bit `(i - firstOptionalArgument)` set for every provided argument from the first optional one onward (so mandatory arguments after the first optional one also occupy bits) (22052-22062);
- one hard-coded exception: the format `[?],?,?` (INSTR, _INSTRREV) — a 2-argument call is shifted right and `NULL` is inserted for the first parameter; the mask is `|1` iff the first argument was given (20261-20274, 22055-22056).

Without a format: argument count must equal `id.args`, except `ASC` with 2 args, and `id.overloaded = -1` with `minargs <= n <= args` (20293-20303); non-overloaded functions get trailing `NULL`s for missing args (21980-21984).

### 4.4 Lambda wrapping

When evaluating an argument needs statements (only the [member-array layer]: building a temporary descriptor for a TYPE member array), the call is wrapped: functions as `([&](){<prep>return f(...);})()` (22064), SUBs as `[&](){<prep>f(...);}();` (12389).


---

## 5. Built-ins: registration and what is special-cased

> Provenance note: the helper agent assigned to this section failed (rate limit), so this section is from my own greps and targeted reads. Counts are grep counts; the special-case list is from the `firstelement$ = "…"` tests and from `evaluatefunc`, and may miss handlers that test other variables.

### 5.1 The id record (`TYPE idstruct`, 560-615)

| Field | Meaning |
|---|---|
| `n` / `cn` | upper-case name / declared-case name (STRING*256) |
| `t`, `tsize` | variable type code; fixed-string length |
| `arraytype`, `arrayelements`, `staticarray` | array element type; dimension count (-1 unknown); static flag |
| `musthave` / `mayhave` | type suffix that must / may follow the name (mutually exclusive). Built-in `MID$` has `musthave = "$"`, which is how a numeric `Mid` and `MID$` can coexist |
| `subfunc` | 1 = function, 2 = sub |
| `internal_subfunc` | set by `regid` from global `reginternalsubfunc` — built-in (affects name-clash rules and layout casing) |
| `callname` | C function to call (`func_…` / `sub_…`), or `SUB_NAME`/`FUNC_NAME`, or a variable's C name |
| `ccall` | DECLARE LIBRARY / OpenGL: C calling rules (4.2) |
| `args`, `arg` | parameter count; `MKL$(typecode)` per parameter (max 100). Negative codes -1…-8 are pseudo-types (4.1; header comment `subs_functions.bas:4-28`) |
| `argsize` | per-parameter fixed-string size |
| `nele`, `nelereq` | per-parameter array dimension counts |
| `overloaded`, `minargs` | accept `minargs..args` arguments without a format |
| `specialformat` | syntax template (4.3) |
| `secondargmustbe` / `secondargcantbe` | disambiguate same-named subs by the *second element of the statement* (`findid` 23336-23350) |
| `ret` | function return type code (no `ISPOINTER` except strings); `ISUDT + (1)` = returns a `_MEM` (`mem_block`) |
| `Dependency` | `DEPENDENCY_*` index switched on when the built-in is used (`SetDependency`), controlling which runtime parts are linked |
| `hr_syntax` | human-readable syntax for error messages |
| `insubfunc`, `insubfuncn`, `share`, `staticscope` | scope data filled by `regid` |
| `linkid`, `linkarg`, `sfid`, `sfarg` | parameter <-> procedure links |
| `dynudt`, `dynudtmode`, `dynudtargmode` | [member-array layer] |

`regid` (25834-26066) appends to `ids()` (doubling), stamps scope, performs name-clash checks unless `reginternalsubfunc`/`reginternalvariable`, and `HashAdd`s the name with flags `HASHFLAG_SUB/FUNCTION/VARIABLE/ARRAY`. `findid` (23254) walks the hash chain; callers loop with `findanotherid = 1` while it returns 2.

### 5.2 Registration table (`SUB reginternal`, `subs_functions.bas`)

Each built-in is a block of assignments ending in `regid`:

```basic
clearid
id.n = "_Resize"
id.subfunc = 2
id.callname = "sub__resize"
id.args = 2
id.arg = MKL$(LONGTYPE - ISPOINTER) + MKL$(LONGTYPE - ISPOINTER)
id.specialformat = "[{On|Off}][,{_Stretch|_Smooth}]"
id.hr_syntax = "_RESIZE [{ON|OFF}][, {_STRETCH|_SMOOTH}]"
regid                                              ' subs_functions.bas:135-143
```

Counts (grep over `subs_functions.bas`):

| Measure | Count |
|---|---|
| registrations (`id.subfunc =`) | 455 (427 block-style + 28 one-line stubs) |
| functions / subs | 289 / 166 |
| with `specialformat` | 179 |
| `_`-prefixed names | 294 |
| with a `Dependency` | 68 (MINIAUDIO 30, DEVICEINPUT 13, LOADFONT 7, ZLIB 6, PRINTER 3, SCREENIMAGE 3, IMAGE_CODEC 2, SOCKETS 2, EMBED 1, ICON 1) |
| `sub_stub`/`func_stub` entries | 28 |
| `overloaded` | 1 (`_RGB32`, `minargs = 1`, lines 1702-1706) |
| `secondargmustbe`/`cantbe` | `Shell` (`_Hide`, `_DontWait`), `Get`/`Put` (`Step`, `(` = graphics forms), `Def` (`Seg`), `View` (`Print`) |

Return types: LONG 122, STRING 65, _FLOAT 33, DOUBLE 21, _UNSIGNED LONG 13, _UNSIGNED _INTEGER64 7, SINGLE 6, `-1` 6 (placeholder, overridden in `evaluatefunc`), `ISUDT+(1)` 5 (_MEM), _INTEGER64 4, INTEGER 3, _UNSIGNED _OFFSET 2.

Many names are registered twice (statement and function): `_Resize`, `Timer`, `Screen`, `Play`, `_Font`, `_Blend`, `_ClearColor`, `_PrintMode`, `_PaletteColor`, `Input`, `_MessageBox`, `Strig`, `Key`. Several statements have multiple variants selected by `secondarg*` or by trying each format in turn (`Get` x4, `Put` x4, `Open` x2, `Screen` x4 at 3386-3389, `Width` x2, `View` x2, `Shell` x3): the generic path loops over all ids of that name and moves to the next one on a format mismatch (12419-12427).

**Stubs**: names registered with `callname = "sub_stub"` only so that they are reserved and found; their real handling is hard-coded (`Asc`, `End`, `LSet`, `RSet`, `Mid$`, `Print`, `Option`, `Swap`, `System`, `Write`, `Read`, `Close`, `Reset`, `Input`, `_MemGet`, `_MemPut`, `_MemFill`, `_Continue`; `subs_functions.bas:31-88`). If one reaches the generic path it yields "Command not implemented" (11567 / 20237); that is also the designed behaviour for the unimplemented QB commands `TrOn`, `TrOff`, `List`, `Def` (non-SEG), `IoCtl`, `IoCtl$`, `Fre`, `SetMem`, `FileAttr`.

**OpenGL** (`extensions\opengl\opengl_methods.bas`): not hand-written. `gl_scan_header` (67) parses a GL header into `GL_COMMANDS()`/`GL_DEFINES()`; `gl_include_content` (331-404) registers each command with `id.ccall = 1` and callname = the C GL function, each `#define` as a global `_GL_…` INTEGER64 constant, plus one hand entry `_gluPerspective`. I did not count them.

**`syntax_highlighter_list.bas`**: a hand-maintained `@`-delimited string `listOfKeywords$` (metacommands, keywords A-Z, each in QB64/QB4.5/OpenGL groups) used by the IDE (`ide_methods.bas:13409`, `ide_export.bas:388`). It is independent of the id table; nothing keeps it in sync automatically. User SUB/FUNCTION names are appended to `listOfCustomKeywords$` by `regid` (25900-25904).

`$NOPREFIX` is no longer supported: it is a compile error telling the user to convert (1085, 2020-2026). There is one registration per name; no un-prefixed aliases.

### 5.3 Distinct `specialformat` strings (first occurrence, verbatim)

| Line | Statement/function | Format |
|---|---|---|
| 224 | `_Mem` (func) | `?[,?]` |
| 298 | `_MemCopy` | `?,?,?{To}?,?` |
| 347 | `_Console` | `{On|Off}` |
| 429 | `Strig` | `[(?[,?])]{On|Off|Stop}` |
| 548 | `Key` | `(?){On|Off|Stop}` |
| 574 | `_ScreenMove` | `[{_Middle}][?,?]` |
| 638 | `_MapUnicode` | `?{To}?` |
| 721 | `_PrintImage` | `[_SQUAREPIXELS][?][,(?,?)-(?,?)]` (keyword spelled as literal characters, not in `{}`; looks accidental) |
| 763 | `Lock` | `[#]?[,[?][{To}?]]` |
| 791 | `Timer` | `[(?)]{On|Off|Stop|Free}` |
| 811 | `_AllowFullScreen` | `[{_Stretch|_SquarePixels|_Off|_All|Off}][,{_Smooth|_Off|_All|Off}]` |
| 845 | `_WindowSizeLimit` | `[(?,?)][-(?,?)]` |
| 856 | `_Clipboard$ =` | `=?` |
| 1163 | `Clear` | `[?][,[?][,?]]` |
| 1310 | `_PutImage` | `[[{Step}](?,?)[-[{Step}](?,?)]][,[?][,[?][,[[{Step}](?,?)[-[{Step}](?,?)]][,{_Smooth}]]]]` |
| 1320 | `_MapTriangle` | `[{_Clockwise|_AntiClockwise}][{_Seamless}](?,?)-(?,?)-(?,?)[,?]{To}(?,?[,?])-(?,?[,?])-(?,?[,?])[,[?][,{_Smooth|_SmoothShrunk|_SmoothStretched}]]` |
| 1330 | `_DepthBuffer` | `{On|Off|Lock|_Clear}[,?]` |
| 1340 | `_SetAlpha` | `?[,[?[{To}?]][,?]]` |
| 1518 | `_PrintString` | `[{Step}](?,?),?[,?]` |
| 1942 | `Name` | `?{As}?` |
| 1997 / 2008 | `Shell` | `{_Hide}[{_DontWait}][?]` / `{_DontWait}[{_Hide}][?]` |
| 2101 | `Sound` | `[?,?[,[?][,[?][,[?][,[?][,?]]]]]][{Wait|Resume}]` |
| 2376 | `Seek` | `[#]?,?` |
| 2517 | `LBound`/`UBound` | `?,[?]` |
| 2699 | `Paint` | `[{Step}](?,?)[,[?][,[?][,?]]]` |
| 2710 | `Circle` | `[{Step}](?,?),?[,[?][,[?][,[?][,?]]]]` |
| 2741 | `Get` (graphics) | `[{Step}](?,?)-[{Step}](?,?),?[,?]` |
| 2764 | `Put` (graphics) | `[{Step}](?,?),?[,[{_Clip}][{PSet|PReset|And|Or|Xor}][,?]]` |
| 2790-2791 | `Get` (file) | `[#]?,[?],?` and `[#]?[,[?][,?]]` |
| 2812 | `Open` | `?[{For Random|For Binary|For Input|For Output|For Append}][{Access Read Write|Access Read|Access Write}][{Shared|Lock Read Write|Lock Read|Lock Write}]{As}[#]?[{Len =}?]` |
| 2822 | `Open` (old form) | `?,[#]?,?[,?]` |
| 3013 | `InStr`, `_InStrRev` | `[?],?,?` |
| 3036 | `Mid$` (func) | `?,?,[?]` |
| 3147 | `Def Seg` | `{Seg}[=?]` |
| 3245 | `Line` | `[[{Step}](?,?)]-[{Step}](?,?)[,[?][,[{B|BF}][,?]]]` |
| 3285 | `Randomize` | `[[{Using}]?]` |
| 3313 / 3324 | `View` / `View Print` | `[[{Screen}](?,?)-(?,?)[,[?][,?]]]` / `{Print}[?{To}?]` |
| 3335 | `Window` | `[[{Screen}](?,?)-(?,?)]` |
| 3345 | `Locate` | `[?][,[?][,[?][,[?][,?]]]]` |
| 3355 | `Color` | `[?][,[?][,[?][,?]]]` |
| 3375-3376 | `Width` | `[{#|LPrint}][?][,[?][,[?][,[?]]]]` / `[{#|LPRINT}][?][,?]` |
| 3387-3389 | `Screen` | `[?][,[?][,[?][,[?][,{_MANUALDISPLAY}]]]]` and two variants |
| 3399 | `PSet`/`PReset` | `[{Step}](?,?)[,?]` |
| 4159 | `_CapsLock` etc. | `{On|Off|_Toggle}` |

So LINE, CIRCLE, PAINT, PSET, graphics and file GET/PUT, OPEN, LOCATE, COLOR, SCREEN, VIEW, WINDOW **are** table-driven through the mini-language; multi-word keyword alternatives (`For Random`, `Lock Read Write`, `Len =`) are matched as element sequences.

### 5.4 Statements with dedicated code (not, or not only, table-driven)

Main pass of `qb64pe.bas`, in source order. The generic sub-call lookup starts at 10809; special cases inside the id loop run before the generic argument code at 11561.

| Statement | Line | Notes |
|---|---|---|
| `SUB`/`FUNCTION`/`END SUB` | 5329-6020 | definition |
| `DECLARE [LIBRARY]`, `END DECLARE` | 4445-5320 | |
| `TYPE … END TYPE` | 4434 (main), 2483 (pre-pass) | |
| `CONST` | 6047 (main), 2530 (pre-pass) | evaluated only in the pre-pass |
| `DEFINT…DEFSTR`, `_DEFINE` | 6113-6175 | |
| `NEXT`, `WHILE`, `WEND`, `DO`, `LOOP`, `FOR` | 6212, 6274, 6311, 6330, 6378, 6435 | |
| `ELSE`, `ELSEIF`, `IF`, `END IF` | 6594, 6644, 6686 | |
| `SELECT [EVERYCASE] CASE`, `END SELECT`, `CASE` | 6790, 6900, 6947 | |
| `PALETTE USING` | 7238 | |
| `KEY …` forms | 7281 | |
| `FIELD` | 7340 | |
| `EXIT …` | 7445, 8313 | |
| `ON STRIG/TIMER/KEY …` | 7534, 7726, 7907 | |
| `SHARED` (in procedure) | 8060 | |
| `_ECHO` | 8332 | |
| `ASC(…) =` | 8341 | |
| `MID$(…) =` | 8449 | |
| `ERASE` | 8529 | |
| `DIM`, `REDIM [_PRESERVE]`, `STATIC`, `COMMON` | 8770-8786 | -> `dim2`/`allocarray` |
| `_CONTINUE` | 9735 | |
| `CHAIN`, `RUN` | 9758, 9764 | |
| `END`, `SYSTEM`, `STOP` | 9846, 9870, 9905 | |
| `GOTO` | ~9686-9729 | |
| `GOSUB`, `RETURN` | 9929, 9938 | |
| `RESUME` | 9987 | |
| `ON ERROR GOTO` | ~10050-10111 | |
| `RESTORE` | 10117 | |
| `ON n GOTO/GOSUB` | 10160 | `xongotogosub` |
| `_MEMGET`, `_MEMPUT`, `_MEMFILL` | 10170, 10267, 10406 | |
| `_ARRAYCOPY` | 10521 | [fork] `arrcpy.bm` |
| `CALL INTERRUPT[X]`, bare `INTERRUPT` | 10606-10677 | `call_interrupt(n, byte_element, byte_element);` |
| `CALL ABSOLUTE` | 10687-10776 | `call_absolute_offsets[i]=…; call_absolute(argc, offset);` |
| `CALL name[(…)]` | 10617-10806 | rewritten to a plain sub call |
| `?` | 10811 | alias of PRINT |
| `_GL` block, `VWATCH` | 10863, 10867 | |
| `OPEN` (pre-processing) | 10871 | then table formats |
| `CLOSE` / `RESET` | 10893 | |
| `READ` | 10972 | `xread` 27936 |
| `LINE INPUT` | 11017-11020 | |
| `INPUT` | 11024-11240 | |
| `WRITE` / `WRITE #` | 11241-11256 | `xwrite` 27993, `xfilewrite` 27486 |
| `PRINT #` / `PRINT` / `LPRINT` | 11257, 11269 | `xfileprint` 27258, `xprint` 27713 |
| `CLEAR` (argument sanity) | 11318 | then generic |
| `LSET` / `RSET` | 11327 | |
| `SWAP` | 11377 | |
| `OPTION BASE / _EXPLICIT / _EXPLICITARRAY` | 11507 | |
| `_LOGTRACE/_LOGINFO/_LOGWARN/_LOGERROR` | 11537-11559 | `EmitLoggingStatement` |
| **generic sub call** | 11561-12414 | everything else, user SUBs included |
| inside generic: file `GET`/`PUT` | 11622-11638, 11880-11951 | `sub_get2`/`sub_put2`/`field_get`/`field_put` |
| inside generic: `PAINT` 3rd arg | 12009-12015, 12286-12294 | string tile pattern vs colour: `(qbs*)` / `(uint32)` cast |
| inside generic: `_WAVE`, `_SNDRAWBATCH` | 11810, 11830 | array type checks |
| inside generic: `SLEEP` | 12391-12404 | `$DEBUG` hooks |
| inside generic: `TIMER`/`KEY` | 11640, 11744 | layout spacing only |
| `LET` / assignment | 12433-12455 | `assign` |
| `DATA` | 4256 + `lineformat` 25211 | captured by the tokenizer |

### 5.5 Functions special-cased in `evaluatefunc` (by name)

| Function | Lines | Behaviour |
|---|---|---|
| `VAL(s$[, type])` | 20355-20391, 20695-20715 | `qbs_val<long double>(s)` typed _FLOAT; with a type argument: `qbs_val<int64_t>`, `qbs_val<uint64_t>`, `(float)`/`(double)qbs_val<long double>` |
| `_CAST(type, v)` | 20394-20417, 20718-20734 | `((ctype)(v))`; first argument is a type name |
| `_CV(type, s$)`, `CVI CVL CVS CVD` | 20420-20427, 21255-21291 | `string2i/l/s/d/b/ub/ui/ul/i64/ui64/f/o/uo(s)`, `string2bit(s,n)` |
| `_MK$(type, v)`, `MKI$ MKL$ MKS$ MKD$` | 20431-20441, 21217-21253 | `i2string(` … with the argument coerced to the named type |
| `_EMBEDDED$("handle")` | 20443-20481 | compile-time validation against the `$EMBED` list |
| `UBOUND`/`LBOUND` | 20483-20542, 21987-22050 | first arg evaluated as `name()`; `func_ubound(desc, dim, ndims)`; default dimension via the `",NULL"` -> `",1"` hack ("FIXME: ??????" 21988); result INTEGER64 |
| `INPUT$(n, #f)` | 20546-20552 | strips `#` |
| `ASC(s$, pos)` | 20293, 20556-20564 | 2-arg form bypasses the arg-count check |
| `_MEMGET(m, offs, type)` | 20573-20640 | `*(type*)(offs)` or checked `func__memget(blk, offs, size)`; string/UDT variants |
| `_MEM(var)` / `_MEM(offset, size)` | 20924-20934, 21961-21968 | `-7` pseudo-arg / `func__mem_at_offset` |
| `_OFFSET(var)` | 20937-20948 | address expression |
| `ENVIRON$(n or s$)` | 20952-20958 | accepts either type |
| `LEN(x)` | 20961-20974 | string expr: `((int32)(e)->len)`; variable: size via `-5` |
| `_BIN$`, `OCT$`, `HEX$` | 20978-21062 | digit count derives from the operand *type* (`func_hex(v, chars)`); a 64-bit non-variable expression passes 0 = minimal width; floats use `_float` variants |
| `EXP` | 21066-21084 | `func_exp_single` (<=16-bit ints and SINGLE) vs `func_exp_float` |
| `INT`, `FIX`, `_ROUND`, `CDBL`, `CSNG`, `CLNG`, `CINT` | 21087-21215 | inline casts / range-checked helpers; `INT`/`FIX` keep the argument type. `CSNG` of an integer emits `((double)(e))` (21164): typed SINGLE but not narrowed |
| `STRING$(n, s$)` | 21294-21303 | |
| `SADD`, `VARPTR`, `VARPTR$`, `VARSEG` | 21306-21560 | cmem (3.3) |
| `_IIF(c, a, b)` | 20737-20768 | C ternary `((c)?(a):(b))`; only the selected branch is evaluated; result type by `Type_PromoteArithmeticType` |
| `_MIN`, `_MAX`, `_CLAMP` | 20771-20850 | `std::min<T>(a,b)`, `func_clamp<T>` with promoted T |
| `_ROR`, `_ROL` | 20853-20900 | `func__ror<uintN_t>(v, n)`; width from the first operand; returns unsigned |
| `_UCHARPOS` | 20903-20921 | 2nd arg must be a LONG array |
| `ABS` | 22068 | returns the argument's type |
| `SIN COS TAN ATN SQR LOG` | 22071-22082 | result type by argument: SINGLE for <=16-bit ints/SINGLE, DOUBLE for 32-bit ints/DOUBLE, else _FLOAT |
| any function returning `_MEM` | 22084-22089 | result stored in a `mem_block funcN` temp |

Operators (`AND`, `MOD`, `NOT`, `_ANDALSO`, …) are not ids; they are handled by `isoperator`/`operatorusage`.

---

## 6. Control flow emission

(From a helper agent's read of the main loop; I checked the statement epilogue at 12481-12489 and `closemain` 17568-17607 myself.)

Most emission goes through `WriteBufLineCpp`, which writes a `#line N "file"` directive before the text (`source\utilities\s-buffer\sb_qb64pe_extension.bm:237-290`).

### 6.1 Per-statement wrapper and error/event checks

`statementn` increments per executable statement (6195).

| Statement class | Prologue | Epilogue |
|---|---|---|
| ordinary | `do{` (7231) | `if(!qbevent)break;evnt(N);}while(r);` (12487) |
| block openers: WHILE, DO, LOOP-with-condition, FOR, IF, ELSEIF, SELECT, CASE | `S_<n>:;` (e.g. 6275) | `if(qbevent){evnt(N);if(r)goto S_<n>;}` (12485) |
| NEXT, WEND, bare LOOP, ELSE, END IF, END SELECT, CASE ELSE | none | none |

`N` is the source line (plus `,incline,"file"` inside includes, 12474-12480). `qbevent` is a global set by `error()` and by the timer thread; `evnt()` (`qbx.cpp:1508-1541`) handles a pending error or dispatches events and sets `r` (retry) for RESUME. Runtime functions do not throw: after `error(n)` they return a dummy value and the statement continues, which is why emitted code is full of `if (!is_error_pending())` guards and why conditions are written `if ((cond)||is_error_pending()){`.

`$CHECKING:OFF` (`CheckingOn = 0`, 3563): no `do{`/`S_n`/`evnt`, no label event checks, `array_check` removed, `_MEM` accesses unchecked. The `||is_error_pending()` terms and the sub-entry guard remain. So there is no event polling and no ON ERROR recovery in such regions.

With `$ErrorLocation` mode 2 each statement starts with `error_track_line(line,incline,"file");` (6201-6209). `$DEBUG` adds `*__LONG_VWATCH_LINENUMBER= N; SUB_VWATCH(…)` hooks.

### 6.2 Labels, GOTO, GOSUB/RETURN

- Label: `LABEL_<name>:;` (3974, 4038). `validlabel` (27121) normalises: upper-case; numeric labels keep their spelling with `.`->`p`, trailing `#`->`d`, `!`->`s`. A numeric label also emits `last_line=<number>;` (3977); this is `ERL`. With checking on, a label is followed by `if(qbevent){evnt(N);r=0;}`.
- Labels are scoped per procedure; undefined/ambiguous labels are diagnosed in a post-pass (13011-13038).
- `GOTO x` -> `goto LABEL_x;` (9729).
- `GOSUB x` (`xgosub` 27574):

```c
return_point[next_return_point++]=G;
if (next_return_point>=return_points) more_return_points();
goto LABEL_x;
RETURN_G:;
```

  and `case G: goto RETURN_G; break;` is appended to `retK.txt` (K = procedure number, G = program-wide counter from 1).
- `RETURN` -> `#include "retK.txt"` pasted inline (9940): `if (next_return_point){ next_return_point--; switch(return_point[next_return_point]){ case 0: return; … } } error(3);` (in a procedure, `case 0` is `error(3)`). Id 0 is pushed by event dispatch so that RETURN from an `ON TIMER GOSUB` handler returns from the nested `QBMAIN`.
- `RETURN label` (main only): `if (!next_return_point) error(3); next_return_point--; goto LABEL_x;`.
- `ON n GOTO/GOSUB` (`xongotogosub` 27618): `static int32 ongo_U=0; ongo_U=<expr>;` then `if (ongo_U==k) goto LABEL_x;` per target (the GOSUB form wraps the GOSUB sequence), then `if (ongo_U<0) error(5);`. No check for >255.

### 6.3 ON ERROR / RESUME

- `ON ERROR GOTO lbl` -> `error_goto_line=E;` in main and `if (error_goto_line==E){error_handling=1; goto LABEL_x;}` in `mainerr.txt` (10110-10111). `ON ERROR GOTO 0` -> `error_goto_line=0;`.
- Runtime: `error(n)` sets `new_error` and `qbevent`; the statement epilogue calls `evnt` -> `fix_error()`, which (if a handler exists and none is active) sets `error_err`, `error_erl=last_line`, `error_occurred=1` and **calls `QBMAIN(NULL)` recursively**; `mainerr.txt` at the top of QBMAIN jumps to the handler (`internal\c\libqb\src\error_handle.cpp:425-431`).
- `RESUME` -> `if (!error_handling){error(20);}else{error_retry=1; qbevent=1; error_handling=0; error_err=0; return;}`: the nested QBMAIN returns into `evnt`, `r=1`, and the faulting statement's `do{…}while(r)` / `goto S_n` re-executes it. `RESUME NEXT` is the same without `error_retry` (`r=0`). `RESUME label` -> `error_handling=0; error_err=0; goto LABEL_x;` and never unwinds the nested call.
- Retry/next granularity is exactly one emitted statement wrapper. Because conditions are `(cond)||is_error_pending()`, `RESUME NEXT` after an error in an IF/WHILE/CASE condition continues *inside* the block.
- Errors 11 (division by zero), 256, 257, 259-261, 270, 271, 502-518 are fatal immediately and cannot be trapped (`error_handle.cpp:443-497`).

### 6.4 Events

`ON TIMER(n) GOSUB lbl` (7756-7828): `ontimer_setup(i,sec,ID,0);` in main; `if(timer_event_id==ID)goto LABEL_x;` in `ontimerj.txt`; a `case ID:` in `ontimer.txt` that pushes return id 0 and calls `QBMAIN(NULL)`. `ON TIMER(n) subname` calls the SUB directly. `ON KEY` and `ON STRIG` are analogous (`onkey*.txt`, `onstrig*.txt`).

### 6.5 Structured statements

Control stack: `controllevel`, `controltype()` (1 IF, 2 FOR, 3 DO, 4 DO WHILE/UNTIL, 5 WHILE, 6 `$IF`, 10-17 SELECT by selector C type, 18 CASE, 19 CASE ELSE, 32 SUB/FUNCTION), `controlid()`, `controlvalue()`, `controlstate()` (768-790).

| BASIC | C++ |
|---|---|
| `IF e THEN` | `if ((e)||is_error_pending()){` (6727); with string temps `if ((qbs_cleanup(qbs_tmp_base,e))||is_error_pending()){` (6725) |
| `ELSEIF e THEN` | `}else{` + `if (e){`; one extra `}` owed per ELSEIF (`controlvalue`); no `is_error_pending` term |
| `ELSE` / `END IF` | `}else{` / `}` x (1 + number of ELSEIFs) |
| single-line IF | rewritten by the line splitter (4108-4178) to block form with an implied `END IF` |
| `WHILE e` … `WEND` | `while((e)||is_error_pending()){` … `ww_continue_C:;` `}` `ww_exit_C:;` |
| `DO WHILE e` / `DO UNTIL e` / `DO` | `while((e)||is_error_pending()){` / `while((!(e))||is_error_pending()){` / `do{` |
| `LOOP` / `LOOP WHILE e` / `LOOP UNTIL e` | `dl_continue_C:;` then `}` or `}while(1);` or `}while((e)&&(!is_error_pending()));` or `}while((!(e))&&(!is_error_pending()));` then `dl_exit_C:;` |
| `EXIT DO/WHILE/FOR`, `_CONTINUE` | `goto dl_exit_C;` etc. / `goto …_continue_C;` |
| `EXIT SUB/FUNCTION` | `goto exit_subfunc;` (8321) |
| `EXIT SELECT` / `EXIT CASE` | `goto sc_U_end;` / `goto sc_ec_M_end;` |

**FOR** (6435-6588): the loop variable must be a simple numeric scalar. Temporaries are wider than the variable (SINGLE -> `double`; DOUBLE/_FLOAT -> `long double`; 8-bit -> `int16`; 16-bit -> `int32`; 32/64-bit -> `int64`, including unsigned 64):

```c
fornext_valueU=<start>;
fornext_finalvalueU=<end>;
fornext_stepU=<step>;
if (fornext_stepU<0) fornext_step_negativeU=1; else fornext_step_negativeU=0;
if (is_error_pending()) goto fornext_errorU;
goto fornext_entrylabelU;
while(1){
fornext_valueU=fornext_stepU+(<var>);
fornext_entrylabelU:
<var>=fornext_valueU;                 // via setrefer
if (fornext_step_negativeU){
if (fornext_valueU<fornext_finalvalueU) break;
}else{
if (fornext_valueU>fornext_finalvalueU) break;
}
fornext_errorU:;
   ...body...
fornext_continue_C:;
}
fornext_exit_C:;
```

End and step are evaluated once; the increment re-reads the real variable (body modifications count); the exit test is done on the wider temp, after the variable has already been assigned its narrowed value. The temporaries are `static` in main and plain locals in procedures.

**SELECT CASE** (6790-7209): if the selector is a plain scalar variable it is *referenced directly* and re-read at each CASE; otherwise it is copied into `static <t> sc_U;` (`static qbs *sc_U` for strings). Each CASE is `if ((alt1||alt2…)||is_error_pending()){` with alternatives `(sel==(e))`, `(sel op (e))` for `IS`, `((sel>=(a))&&(sel<=(b)))` for `TO`, and `qbs_equal`-family calls for strings. A case body ends with `goto sc_U_end; }`; END SELECT emits `sc_U_end:;`. `SELECT EVERYCASE` replaces the goto with `sc_U_var=-1;` and CASE ELSE with `if (sc_U_var==0) {`. Float CASE values against integer selectors are rounded (`qbr…`).

### 6.6 DATA / READ / RESTORE

- `lineformat` captures DATA text raw (25211-25309), appends each line's items (comma-terminated, quotes preserved) to `data.bin` and advances `DataOffset`. All DATA of the program, main and procedures, forms **one blob in source order**.
- At the end the blob is inlined into `global.txt` as `ptrszint data_size=N; uint8 inline_data[]={…,0}; uint8 *data=&inline_data[0];` (13114-13139).
- A label records the `DataOffset` in effect at its line; labels used by `RESTORE` get `ptrszint data_at_LABEL_x=<off>;` (13056). `RESTORE` -> `data_offset=0;` / `data_offset=data_at_LABEL_x;`. RESTORE labels are global ("Ambiguous DATA label", 13053).
- `READ` (`xread` 27936): strings `sub_read_string(data,&data_offset,data_size,<qbs>);`, 64-bit ints `func_read_int64/uint64(…)`, everything else `func_read_float(data,&data_offset,data_size,<typ>)`. Parsing is at run time.

### 6.7 Termination

| BASIC | C++ |
|---|---|
| `END` / end of main | `sub_end();` (`xend` 27251), then `return;` |
| `END n` | `exit_code=n;` + `sub_end();` |
| `SYSTEM [n]` | `[exit_code=n;] if (sub_gl_called) error(271); close_program=1; end();` (9897-9899) |
| `STOP` | `close_program=1;end();` |
| `RUN` | `sub_run_init(); sub_clear(NULL,NULL,NULL,NULL);` then `goto S_0;` (main) or `QBMAIN(NULL);` (in a procedure); `RUN label` uses `goto LABEL_x` or `run_from_line=K` + `runline.txt`; `RUN "file"` -> `sub_run(str)` |
| `CHAIN` | `sub_chain` in `qbx.cpp:596`; COMMON values are serialised positionally by generated `chain.txt` / `inpchain.txt` |

---

## 7. Compile-time evaluation (`const_eval.bas`)

(From a helper agent's full read of `const_eval.bas`; not re-read by me.)

`Evaluate_Expression$(e$, num AS ParseNum)` (`const_eval.bas:11`) works on the same `sp`-separated elements and returns a literal element or `"ERROR - …"`. It is used **only** for `CONST` (pre-pass, 2586) and [fork] TYPE member-array bounds (15843). It is a separate interpreter with its own semantics; DIM bounds, CASE values etc. use the normal `evaluate` path plus the `constequation` flag instead.

- Structure: `PreParse` (1088; upper-cases, checks parentheses, brackets `NOT` operands), then repeatedly evaluates the innermost parenthesis group with a recursive-descent parser (`ParseExpression2` 791; grammar comment 103-163) and splices the literal result back as text. Every intermediate value is therefore **re-serialised and re-parsed** per parenthesis level: integers become `n&&`/`n~&&`, floats are re-typed from their printed text (an integer-valued float comes back as INTEGER64).
- Value: `ParseNum {f AS _FLOAT, i AS _INTEGER64, ui AS _UNSIGNED _INTEGER64, s AS STRING, typ AS LONG}` (`const_eval.bi:9-15`).

| Precedence (low -> high) | Operators | Notes |
|---|---|---|
| 1 | string `+` | literal/CONST concatenation only |
| 2-6 | `IMP`, `EQV`, `XOR`, `OR`, `AND` | 64-bit; unsigned if either operand unsigned |
| 7 | `NOT` | |
| 8 | `<> >< >= => <= =< < > =` | -1/0; numeric only (no string comparison) |
| 9 | `+ -` | |
| 10 | `MOD` | |
| 11 | `\` | |
| 12 | `* /` | `/` always _FLOAT |
| 13 | unary `-` | |
| 14 | `^`, `ROOT` | **right-associative** (run time is left-associative); `a ROOT b` is a non-BASIC extension |

Functions (`Set_ConstFunctions` 815): `_PI`, `_ACOS`, `_ASIN`, `_ARCSEC`, `_ARCCSC`, `_ARCCOT`, `_SECH`, `_CSCH`, `_COTH`, `COS`, `SIN`, `TAN`, `LOG`, `EXP`, `ATN`, `SQR`, `_D2R`, `_D2G`, `_R2D`, `_R2G`, `_G2D`, `_G2R`, `ABS`, `SGN`, `INT`, `_ROUND`, `_CEIL`, `FIX`, `_SEC`, `_CSC`, `_COT`, `_RGB32` (1-4 args), `_RGBA32`, `_RGB`, `_RGBA`, `_RED32`, `_GREEN32`, `_BLUE32`, `_ALPHA32`, `_RED`, `_GREEN`, `_BLUE`, `_ALPHA`, `CHR$`, `ASC`. Nothing else (`LEN`, `VAL`, `CINT`, `MKI$`, `_SHL`, string functions) is available. Operands may be already-defined CONSTs (`ParseNumHashLookup&` 714); forward references fail.

Differences from run time that matter for compatibility:

- all integer arithmetic is int64/uint64 with silent wrap; `ABS/SGN/INT/FIX/_ROUND/_CEIL` force an INTEGER64 result (so `ABS(1.5)` is rounded);
- `^` with two integer operands is stored back to an integer; `^` associativity differs (above);
- the angle-conversion helpers use truncated constants, and the `_R2G`/`_G2R` factors appear swapped (`const_eval.bas:1011-1016`);
- a suffix on the CONST name (`CONST x% = 3.7`) retypes and rounds the value but there is **no range check** ("range check required here", 2623); `DEFtype` does not affect CONSTs;
- division by zero is unchecked.

Storage: parallel arrays `constname`, `constcname`, `consttype`, `constinteger`, `constuinteger`, `constfloat`, `conststring`, `constsubfunc` (0 = global), `constdefined` (`utilities\hash.bi:69-84`), hashed with `HASHFLAG_CONSTANT`. `constdefined` is reset before the main pass and set when the CONST line is reached again (3263, 6099), so a constant is invisible above its definition. Substitution into expressions happens in `fixoperationorder` step H (1.3). `STRING * constname` is resolved in `typname2typ&` (`type.bas:617-667`) and `dim2` (17885-17923). DATA is not subject to substitution.

`$IF`/`$LET` (`EvalPreIF` 28352) is a third, purely textual evaluator: comparisons `= <> < > <= >=` between a flag and a word, then `AND`/`OR`/`XOR` strictly left to right; no parentheses, arithmetic or `NOT`; special words `DEFINED`/`UNDEFINED`; presets `WINDOWS WIN LINUX MAC MACOSX 32BIT 64BIT VERSION _QB64PE_ _ARM_` plus `_EXPLICIT_ _EXPLICITARRAY_ _ASSERTS_ _CONSOLE_ _DEBUG_ _SOCKETS_` (59-73, 1714-1722).

### 7.1 Type codes (reference; `source\utilities\type.bi`)

| Flag | Value | Flag | Value |
|---|---|---|---|
| ISSTRING | 1073741824 (bit 30) | ISOFFSETINBITS | 16777216 (24) |
| ISFLOAT | 536870912 (29) | ISARRAY | 8388608 (23) |
| ISUNSIGNED | 268435456 (28) | ISREFERENCE | 4194304 (22) |
| ISPOINTER | 134217728 (27) | ISUDT | 2097152 (21) |
| ISFIXEDLENGTH | 67108864 (26) | ISOFFSET | 1048576 (20) |
| ISINCONVENTIONALMEMORY | 33554432 (25) | UDTMASK | 4095 (the file comment says it was 511) |

Low field = size in bits (8/16/32/64/256, or 1..64 for bits) or the UDT index when ISUDT. The `xTYPE` constants include `ISPOINTER` ("stored in a variable"); expression values use `xTYPE - ISPOINTER`. Fixed-string length travels separately (`id.tsize`, `typname2typsize`, `udtetypesize`). UDT #1 is the built-in `_MEM`. UDT layout is packed, no alignment; member offsets are running sums of `udtesize` (bits). Key helpers: `typname2typ&` (`type.bas:559`), `typ2ctyp$` (425), `type2symbol$` (503), `symbol2fulltypename$` (287), `typevalue2symbol$` (176), `removesymbol$` (722), `Type_PromoteArithmeticType&` (2174), `copy_full_udt` (1986).

DEFtype: `defineaz(1..27)`/`defineextaz(1..27)` (A-Z, `_`), reset to SINGLE at the start of each pass; `DEFINT` etc. are rewritten to `_DEFINE` (6113-6175).

---

## 8. Shape of the generated program

`internal\c\qbx.cpp` is the single translation unit of a user program: runtime headers and globals, then `#include "../temp/<fragment>.txt"` at fixed points. Nothing generated is a standalone `.cpp`.

| Fragment | Content | Included at |
|---|---|---|
| `global.txt` | `qb_safe_idiv`/`qb_safe_mod` templates; pointer declarations of all main-module and STATIC variables; `data_at_LABEL_x`; `data_size`, `inline_data[]`; option flags | `qbx.cpp:500`, file scope |
| `regsf.txt` | prototypes of every user SUB/FUNCTION (the same header text + `);`), DECLARE LIBRARY prototypes/typedefs/`#include`s | `qbx.cpp:501` |
| `clear.txt` | zeroing of every global/static variable for `CLEAR`/`RUN` | `qbx.cpp:513` in `sub_clear` |
| `maindata.txt` | initialisers for main and STATIC variables; DLL loading; `static` temporaries | `qbx.cpp:1588`, top of `QBMAIN` |
| `mainerr.txt` | ON ERROR dispatch | 1589 |
| `runline.txt` | `RUN label` dispatch | 1590 |
| `ontimerj/onkeyj/onstrigj.txt` | event label jumps | 1593-1601 |
| `ontimer/onkey/onstrig.txt` | `case` bodies | in `events()` 1422-1484 |
| `chain.txt`, `inpchain.txt` (+ per-array `chainN`, `inpchainN`) | COMMON save/load | in `sub_chain` 783 / `chain_input` 578 |
| `main.txt` | `#include "main0.txt"` … `"mainN.txt"` + `func__compdate/comptime/compvers` (`closemain` 17592-17604) | 1606 |
| `main0.txt` | body of the main module; closes `QBMAIN` | via `main.txt` |
| `mainK.txt` | one SUB/FUNCTION each | via `main.txt` |
| `dataK.txt`, `freeK.txt`, `retK.txt` | per-procedure locals, cleanup, RETURN switch | from `mainK.txt` |
| `ret0.txt` | main RETURN switch | at each `RETURN` in main |
| `mainfree.txt` | frees for main / STATIC data | **no `#include` found**; appears never used |
| `dyninfo.txt`, `externtypeN.txt`, `icon.rc`, manifest | misc. | |

Runtime structure: `main()` in libqb starts `QBMAIN` and the timer thread on separate threads and runs the window/GLUT loop on the original thread. `QBMAIN(void*)` (`qbx.cpp:1562`) declares the scratch locals (`tmp_long`, `tmp_fileno`, `tqbs`, `qbs_tmp_base`), includes the data/dispatch fragments, calls `chain_input()`, then includes `main.txt`. `main0.txt` begins with `S_0:;` (RUN target) and ends `sub_end(); return; }`. `QBMAIN` is re-entered recursively for error handlers, event GOSUBs and RUN-from-procedure, which is why its prologue is a dispatcher.

A procedure (`mainK.txt`; 5459-5822, 5963-6016):

```c
int32 FUNC_FOO(int32*_FUNC_FOO_LONG_A,qbs*_FUNC_FOO_STRING_S,ptrszint*_FUNC_FOO_ARRAY_LONG_X){
qbs *tqbs;
ptrszint tmp_long;
int32 tmp_fileno;
uint32 qbs_tmp_base=qbs_tmp_list_nexti;
uint8 *tmp_mem_static_pointer=mem_static_pointer;
uint32 tmp_cmem_sp=cmem_sp;
#include "dataK.txt"
mem_lock *sf_mem_lock;
new_mem_lock();
sf_mem_lock=mem_lock_tmp;
sf_mem_lock->type=3;
libqb_check_stack();
if (is_error_pending()) goto exit_subfunc;
   ... statements ...
exit_subfunc:;
free_mem_lock(sf_mem_lock);
#include "freeK.txt"
if ((tmp_mem_static_pointer>=mem_static)&&(tmp_mem_static_pointer<=mem_static_limit)) mem_static_pointer=tmp_mem_static_pointer; else mem_static_pointer=mem_static;
cmem_sp=tmp_cmem_sp;
return *_FUNC_FOO_LONG_FOO;          // strings: qbs_maketmp(v);return v;
}
```

(The parameter names in the signature line are my illustration of the naming scheme; the prologue/epilogue lines are verbatim from the agent's report.)

- All parameters are pointers: `T*`, `qbs*`, `void*` (UDT), `ptrszint*` (array descriptor). The return type is the C value type (`qbs*` for strings).
- Locals are C-local pointers allocated from the `mem_static` bump arena on every call (so recursion works) and released by restoring `mem_static_pointer`; cmem locals by restoring `cmem_sp`. Strings and dynamic arrays are released by `freeK.txt`.
- The function result is an ordinary local named after the function; `freeK.txt` is truncated after it is created so that it is not freed (5486-5507).
- String parameters get a guard in `dataK.txt`: if the incoming qbs is temporary, fixed or read-only it is replaced by a fresh copy; `freeK.txt` copies back into fixed strings and frees the copy (5741-5757). This is where by-reference behaviour for fixed-length string arguments is implemented.
- `STATIC` procedures/variables: declarations go to `global.txt`, initialisers to `maindata.txt`.

DECLARE LIBRARY emission (4445-5320, 5445-5916): static/header libraries -> plain prototype in `regsf.txt` (optionally prefixed by `#include "externtypeN.txt"` holding `extern "C" `, decided by running `nm` on the library); a header -> `#include`; `DYNAMIC` -> `HINSTANCE DLL_x`, `typedef T (CALLBACK* DLLCALL_fn)(…); DLLCALL_fn fn=NULL;` with `LoadLibrary/dlopen` (error 259) and `GetProcAddress/dlsym` (error 260) in `maindata.txt`; `CUSTOMTYPE` -> `typedef T CUSTOMCALL_fn(…); CUSTOMCALL_fn *fn=(CUSTOMCALL_fn*)&alias;`. Parameter C types: BYVAL `T`, otherwise `T*`, UDT `void*`, STRING `char*`.

---

## 9. Quirks to preserve, and things that look accidental

### 9.1 Semantics a rewrite must reproduce

1. The precedence table in 1.3, including `MOD` below `\` below `* /`, unary minus below `^`, `NOT` below comparisons, left-associative `^`.
2. Boolean results are -1/0 (`-(a==b)`); logical operators are bitwise on integers; float operands of bitwise/`MOD`/`\` are banker's-rounded to int64 first.
3. Banker's rounding wherever a float becomes an integer (assignment, array indexes, integer parameters, `CINT`, `CLNG`, `_ROUND`), via the x87 `fistp` path, including rounding through `float` for <=16-bit targets.
4. `&H`/`&O`/`&B` literals are signed by digit count (`&HFFFF` = -1, `&H10000` = 65536&).
5. Unsuffixed float literals: SINGLE if <=7 significant digits, DOUBLE if <=16, else _FLOAT; but the C++ constant emitted is double precision even for SINGLE.
6. Float comparison narrowing to the smaller float type (1.4).
7. `/` on integers yields `long double`; `^` always goes through `long double pow`.
8. Variables distinguished by suffix; `musthave`/`mayhave` lookup; DEFtype default SINGLE; the 4-step name resolution (local without/with default suffix, then global).
9. Auto-creation of undeclared scalars, and of arrays with upper bound 10 per dimension and lower bound from `OPTION BASE`.
10. By-reference argument passing with silent by-value fallback (no copy-back) when the type differs, when the argument is an expression, or when it is wrapped in parentheses.
11. Arrays: column-major layout, descriptor semantics, error 9 on bad subscript (when checking is on), error 10 on re-DIM, the static-vs-dynamic rule.
12. `REDIM _PRESERVE` preserves by *linear position*, not by coordinates.
13. Fixed-length strings are initialised to NUL bytes; variable-length strings in UDTs are pointer-sized slots and are deep-copied on UDT assignment.
14. Statement-granular `RESUME`/`RESUME NEXT`; `ERL` = last numeric label; errors inside conditions fall into the block.
15. One global DATA blob in source order across all procedures; RESTORE label offsets.
16. SELECT CASE on a plain variable re-reads the variable at each CASE.
17. FOR loop: bounds/step evaluated once, variable re-read at increment.
18. `VARPTR`/`VARSEG`/`SADD` address model (segment 80, offsets from `cmem+1280`).
19. The optional-argument ABI of the runtime library (NULL + `passed` bitmask, keyword-choice indexes): a rewrite that keeps libqb must generate exactly the same flag bits (4.3).
20. Integer arithmetic does not trap overflow, and stores truncate silently.

### 9.2 Looks accidental / hazardous

*Update: a few of these were run against the old compiler (`09-verification.md`, `v04_accidental`): the `_BIT * 33`
overlap and the static SELECT temporary in recursion are confirmed; `ON n GOTO` does check negative values (error 5),
only values above 255 fall through. The rest are untested.*

| Item | Where |
|---|---|
| Type "markup" claims int64 for integer `+ - *` but C++ computes in 32-bit `int` for <=32-bit operands (signed overflow UB) | 20075-20083 |
| SINGLE literals emitted as C++ doubles; `CSNG(int)` emits `(double)` | 19721-19725, 21164 |
| `_BIT*n` scalars with n>32 are `int64` but only 4 bytes of cmem are reserved | 18141, 18215 |
| Negative integer literal suffix test uses `<` at the LONG boundary, so `-2147483648` is typed `&&` | 23934 |
| `UBOUND`/`LBOUND` default dimension implemented by string-matching `",NULL"` | 21988 |
| Unsigned/signed same-width argument mismatch aliases the caller's variable through a pointer cast instead of using a temp | 12180-12184, 21784-21788 |
| A non-array variable silently shadows a CONST of the same name | 24000-24023 |
| CONST suffix retyping and `CONST x% = …` have no range check | 24053, 2623 |
| `const_eval`: right-associative `^`, integer-forcing `ABS`, swapped `_R2G`/`_G2R`, undeclared `num` in `LogicalNot` (`const_eval.bas:393`) | section 7 |
| `isvalidvariable` has dead code after an always-true early exit (the author's comment admits it) | 24573 |
| Whole-compilation restart (`recompile`) for cmem placement, array-parameter dimension counts, label/sub ambiguity; relies on stable id numbering | 21314 etc., 13000-13008 |
| UDT member assignment for arrays of UDTs is not guarded by `is_error_pending()` like other array stores | 26928-26936 |
| `mainfree.txt` generated but never included | section 8 |
| `ontimer.txt`/`onkey.txt` increment `*_event_occurred` twice but QBMAIN decrements once | 7822-7824, 7983-7985 |
| `sc_N` / `ongo_N` temporaries are `static` even in recursive procedures | 6839-6878, 27645 |
| FOR over `_UNSIGNED _INTEGER64` uses signed `int64` temporaries | 6513-6522 |
| `ON n GOTO` lacks QB's 0-255 range check | 27618-27711 |
| `ELSEIF` lacks the `||is_error_pending()` term that IF has | 6660-6675 |
| `RESUME label`, `RUN` from a procedure and event handlers never unwind the nested `QBMAIN` (C stack growth) | error_handle.cpp:430, 9774 |
| `_PrintImage` format spells a keyword as literal characters | subs_functions.bas:721 |
| `typ2ctyp$` checks the wrong variable for `~%&`; `symboltype("##")` returns a 64-bit float | type.bas:479, 370 |
| Layout (pretty-printer) generation is interleaved with code generation in every function (`tlayout$`, `separgslayout`) | throughout |

### 9.3 Structural observations for the rewrite

- The "expression IR" is text; type information is a bit-packed LONG; references are `sp3`-separated strings re-parsed by every consumer (the same four-field split is copy-pasted dozens of times). A typed AST with an explicit lvalue node (variable / array element / UDT path) replaces `refer`/`setrefer`/`evaluatetotyp -2…-8`; the pseudo-types map to "address-of", "sizeof", "span to end of object" and "_MEM descriptor" operations on an lvalue.
- The argument-passing logic is duplicated between `evaluatefunc` and the SUB path, and the optional-argument logic exists in two different interpreters; one implementation suffices if the format is parsed once into a small grammar at registration time.
- Emitted C++ depends on C++ implicit conversion rules for arithmetic typing. A rewrite must either emit the same operand C types or encode the resulting types explicitly; this is the highest-risk area for silent numeric differences.
- Evaluation has side effects on compiler state (auto-DIM, `recompile`, dependency flags); these need to become explicit passes.

## 10. Not determined

- Built-in registration was studied by grep and sampling, not read end to end (the delegated study failed); per-built-in argument type tables, a category breakdown (string/math/graphics/…) and the OpenGL entry count were not produced.
- PRINT / INPUT / WRITE / PRINT USING emission (`xprint`, `xfileprint`, `xwrite`, INPUT at 11024-11240) and the OPEN/CLOSE pre-processing were located but not read.
- `dim2` numeric branches 18323-18890 were sampled (BYTE, start of INTEGER, tail of FLOAT) and assumed uniform.
- The [member-array layer] helpers (`UDTDyn*`, `AppendDyn*`, `BuildUDTMemberArg`, `EmitAssignWhole`, the `arrcpy.bm` runtime) were not studied in depth; whether this layer exists upstream was not verified.
- The `evaluatetotyp` `-7` branch (22730-23038) was read only at its ends.
- libqb internals (`sub_end`, `qbs_set`, `func_read_*`, `byte_element`) are interface-only here.
- Sections 6, 7 and most of 8 come from helper-agent reports; apart from the statement epilogue, `closemain` and the `global.txt` prologue I did not re-verify their line references. Nothing was compiled or run; no generated `internal\temp` output was available to compare against.
