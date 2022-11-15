# HexSpec 

A spec for verifying byte buffers

## Example

```
; Simple Open Framing Header
sofh:
.length:        0000003F          ; SOFH message length
.sig:           EB50              ; SOFH Encoding SBE little endian

; Simple Binary Encoding Header
sbe:
.length:        3700              ; SBE Block Length
.template:      6300              ; SBE Template ID
.schema:        5B00              ; SBE Schema ID
.version:       0000              ; SBE Schema Version

; Single Order
single_order:
.cl_ord_id:     @"ORD00006"
.account:       @"ACCT96\0\0"
.symbol:        @"GEM1" 0000
.side:          32                ; encoded for Sell
.transact:      0000000000000000  ; 1970-1-1T00:00:00.000Z Too lazy to put value here
.order_qty:     05000000          ; 5
.ord_type:      31                ; Market
.price:         @ql(100500)       ; 100.500 (fixed point 3 digits)
.stop_px:       0000000000000000  ; Null
.exec_inst:     04                ; Not Held
```

comments start with a `;` and last to the end of a line

labels may begin a line and are postfixed with `:` and describe 

relative labels start with a `.` and are associated with a non relative label (absolute labels)

`._:` and `_:` unset relative labels and absolute labels respectively

sequences that start with `@` are known as interprets and may come in many forms.

| tag | description                 | size (bytes) | example | hex value                        |
|-----|-----------------------------|--------------|---------|----------------------------------|
| "   | A string                    | nonstatic    | @"GEM1" | 47454D31                         |
| b   | A byte                      | 1            | @b(1)   | 01                               |
| wl  | A word little endian        | 2            | @wl(4)  | 0400                             |
| wb  | A word big endian           | 2            | @wb(4)  | 0004                             |
| dl  | A double word little endian | 4            | @dl(16) | 10000000                         |
| db  | A double word big endian    | 4            | @db(15) | 0000000F                         |
| ql  | A quad word little endian   | 8            | @ql(3)  | 0300000000000000                 |
| qb  | A quad word big endian      | 8            | @qb(2)  | 0000000000000002                 |
| zp  | Zero Pad                    | nonstatic    | @zp(16) | 00000000000000000000000000000000 |

All other tokens are hex bytes.  Stray nibbles (a single hex code/half a byte) maybe not be left on a line.

## Usage

### Rust Api

```
#[test]
fn some_spec() {
  let spec = HexSpec::load(&PathBuf::from("some_spec.hexspec")).unwrap();
  let some_buffer = todo!();

  spec.validate(&some_buffer);
}
```

### Cli support

#### Byte generator

Hexspec can be used to describe a byte buffer in a human readable way.

```
hexspec bytes -f sample.hexspec -o sample.bin
```

will generate the bytes described by sample.hexspec and put them into sample.bin

#### Verfy

Hexspec was made to verify byte buffers and you can do that on the command line.

```
hexspec verify -f sample.hexspec -i sample.bin
```

#### Fmt

hexspec has a default format

```
# format a hexspec stream
cat sample.hexspec | hexspec fmt

# format a hexspec file
hexspec fmt -w sample.hexspec
```

## Editor Support 

There is a vim plugin for syntax support in /editors/vim
