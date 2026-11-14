# TYPE

`TYPE … END TYPE` defines a user type: a record of named members, such as an army with a leader, a number of men
and a morale. A variable of the type is declared with `DIM v AS name`, and its members are written `v.member`.

## Syntax

```text
TYPE name
    member AS type
    member(bounds) AS type
    AS type member [, member]...
END TYPE
```

## Behavior

A member may be of any numeric type but `_BIT`, a `STRING`, a `STRING * n`, or a user type defined in an earlier
`TYPE` block. It may be an array with constant bounds.
<!-- src: language/arrays-and-types "User types"; language/arrays-and-types "Member arrays" -->

The type belongs to the whole program wherever its block stands: in the main module, in a SUB or FUNCTION, or inside
another block. It may be used above its block.
<!-- src: language/arrays-and-types "User types" -->

`DIM v AS name` declares a variable of the type. Every member starts at 0, as an empty string, or as NUL bytes for a
fixed-length string. A member's size is its type's, with no padding between members: `LEN` of a type of a
`STRING * 10`, a LONG and a SINGLE is 18.
<!-- src: language/arrays-and-types "User types"; tests\reference\examples\type_1 -->

`v.member` reads or stores a member, and `a(i).member` a member of an array element. A member is passed by reference
to a parameter of its exact type.
<!-- src: language/arrays-and-types "Member access and dotted names"; tests\reference\examples\type_1 -->

`q = p` copies a whole value of one type to another place of the same type, every member included. `SWAP` exchanges
two.
<!-- src: language/arrays-and-types "Whole user-type values"; tests\reference\examples\type_1 -->

`a.b` is a member access only when a variable `a` of a user type is visible. Otherwise it is a plain variable named
`a.b`.
<!-- src: language/arrays-and-types "Member access and dotted names" -->

A member may have the name of a keyword or a built-in, such as `Time`, `Left` or `Case`.
<!-- src: language/arrays-and-types "User types" -->

Below `$UNSTABLE:TYPEFIELDS`, a member array may be marked `_STATIC` or `_DYNAMIC`. A `_DYNAMIC` member array is an
array of its own in each value, which `REDIM v.items(…)` can give new bounds. `_STATIC` changes nothing.
<!-- src: language/arrays-and-types "Member arrays"; language/arrays-and-types "Dynamic member arrays"; tests\reference\examples\meta-unstable_1 -->

## Errors

These are compile errors:

- A member of type `_BIT`: "a `TYPE` member cannot be a `_BIT`". A member with a type suffix: "a `TYPE` member
  cannot have a type suffix: `a%`".
  <!-- src: language/arrays-and-types "User types" -->
- A second `TYPE` block of a name: "name already in use: `t`".
  <!-- src: language/arrays-and-types "User types" -->
- A member the type does not have: "`Z` is not a member of `TYPE T` (`v.z`)".
  <!-- src: language/arrays-and-types "Member access and dotted names" -->
- A whole value used as a number, as in `PRINT v`: "a whole `TYPE` value cannot be used here: `v`".
  <!-- src: language/arrays-and-types "User types" -->
- A `_STATIC` or `_DYNAMIC` marker without the flag: "`_DYNAMIC` and `_STATIC` members need `$UNSTABLE:TYPEFIELDS`
  above the `TYPE` block".
  <!-- src: language/arrays-and-types "Member arrays"; language/unstable-features "$UNSTABLE flags" -->

## Differences

From QB64pe: a store into a member of an element whose index is out of range raises error 9 there and then writes
element 0. QB64Rust stores nothing (`DIVERGENCES-QB64PE.md` D-004).
<!-- src: D-004 -->

From QuickBASIC 4.5: in `a(setn(1)).k = n`, where the FUNCTION `setn` changes `n`, the index is evaluated first
there, so the new `n` is stored. QB64Rust, as QB64pe, takes the value first for a number member and stores the old
`n`. A string member and a plain array take the index first in all of them (`DIVERGENCES-QB45.md` Q-028).
<!-- src: Q-028 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:type_1 -->
```qb64
TYPE Army
    leader AS STRING * 10
    men AS LONG
    morale AS SINGLE
END TYPE
DIM a AS Army, corps(1 TO 2) AS Army
a.leader = "Davout": a.men = 26000: a.morale = 0.9
corps(1) = a
corps(2).leader = "Lannes": corps(2).men = 20000
PRINT corps(1).leader; corps(1).men; corps(1).morale
PRINT corps(2).leader; corps(2).men
PRINT "An Army takes"; LEN(a); "bytes"
END
```

Output:

```text
Davout     26000  .9 
Lannes     20000 
An Army takes 18 bytes

Press any key to continue
```
<!-- end generated -->

## See also

[`DIM`](dim.md), [`$UNSTABLE`](meta-unstable.md), [`LEN`](len.md), [`_MEM`](_mem.md).
