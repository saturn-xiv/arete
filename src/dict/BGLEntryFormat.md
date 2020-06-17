BLOCK_A(x)
```
x byte(s) as Integer N, followed by N byte(s) of data
```

BLOCK_B(x)
```
1 byte specifier, followed by x bytes of data
```

ENTRY

HEADWORD

DEFINITION
```
ends with 0x14
```

DEFINITION_META
```
\x02: 1 byte WORD_CLASS
\x06: 1 byte UNKNOWN (possibly concerning alternatives)
\x18: BLOCK(1) as DISPLAY_WORD
\x13: 1 byte UNKNOWN
\xC7: no data UNKNOWN
\x50: 1 byte specifier, followed by BLOCK_A(1)
\x60: 1 byte specifier, followed by BLOCK_A(2)
\x4N: BLOCK_B(N+1)
SPECIFIERS:
\x1b: PHONETICS
\x18: WORD_VARIATION (to be studied)
```
DEFINITION_TAIL