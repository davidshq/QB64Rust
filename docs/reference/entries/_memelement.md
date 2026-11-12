# _MEMELEMENT

`_MEMELEMENT(reference)` gives the block of one element or value alone, where `_MEM` of an element reaches to the
array's end.

## Syntax

<!-- begin generated: syntax -->
```text
_MEMELEMENT(referenceVariable)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A `_MEM`.

## Behavior

For `DIM b(10) AS INTEGER`, `_MEM(b())` is 22 bytes, `_MEM(b(4))` 14 and `_MEMELEMENT(b(4))` 2.
<!-- src: language/mem-blocks "A block of a reference"; tests\reference\examples\_memelement_1 -->

## Errors

- A reference QB64pe rejects is a compile error with its message.
  <!-- src: language/mem-blocks "A block of a reference" -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_memelement_1 -->
```qb64
DIM b(10) AS INTEGER, m AS _MEM
m = _MEM(b())
PRINT "The whole array:"; m.SIZE; m.TYPE; m.ELEMENTSIZE
m = _MEM(b(4))
PRINT "From b(4) to the end:"; m.SIZE
m = _MEMELEMENT(b(4))
PRINT "b(4) alone:"; m.SIZE
END
```

Output:

```text
The whole array: 22  65666  2 
From b(4) to the end: 14 
b(4) alone: 2 

Press any key to continue
```
<!-- end generated -->

## See also

`_MEM`.
