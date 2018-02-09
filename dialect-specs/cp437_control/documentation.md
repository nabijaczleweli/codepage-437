[`cp437_DOSLatinUS`](http://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/PC/CP437.TXT) as provided by the Unicode Consortium.

Contains control characters in the `'\x00'..'\x20'` area.

The decode table is additionally enriched from the the [variant table](https://en.wikipedia.org/wiki/Code_page_437#Notes) on Wikipedia.

# Examples

Decoding:

```rust
# use codepage_437::CP437_CONTROL;
assert_eq!(CP437_CONTROL.decode(0x41), 'A');
assert_eq!(CP437_CONTROL.decode(0x91), 'æ');  // LATIN SMALL LIGATURE AE
```

Encoding:

```rust
# use codepage_437::CP437_CONTROL;
assert_eq!(CP437_CONTROL.encode('A'), Some(0x41));
assert_eq!(CP437_CONTROL.encode('æ'), Some(0x91));  // LATIN SMALL LIGATURE AE

assert_eq!(CP437_CONTROL.encode('ź'), None);        // LATIN SMALL LETTER Z WITH ACUTE
```
