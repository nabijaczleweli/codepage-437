cp437 [with wingdings](https://en.wikipedia.org/wiki/Code_page_437#Character_set), as seen on Wikipedia.

Contains wingdings in the `'\x00'..'\x20'` area.

The decode table is additionally enriched from the the [variant table](https://en.wikipedia.org/wiki/Code_page_437#Notes).

# Examples

Decoding:

```rust
# use codepage_437::CP437_WINGDINGS;
assert_eq!(CP437_WINGDINGS.decode(0x41), 'A');
assert_eq!(CP437_WINGDINGS.decode(0x02), '☻');  // BLACK SMILING FACE
```

Encoding:

```rust
# use codepage_437::CP437_WINGDINGS;
assert_eq!(CP437_WINGDINGS.encode('A'), Some(0x41));
assert_eq!(CP437_WINGDINGS.encode('☻'), Some(0x02));  // BLACK SMILING FACE

assert_eq!(CP437_WINGDINGS.encode('ź'), None);        // LATIN SMALL LETTER Z WITH ACUTE
```
