use crate::*;

#[test]
fn duplicate_ignore_abs_labels() {
    HexSpec::parse(
        r"
length: 0000000F
_:      0000
tag:    0001
_:      A5A5A5A5
",
    )
    .unwrap();
}

#[test]
fn duplicate_ignore_rel_labels() {
    HexSpec::parse(
        r"
header:
.length: 0000000F
._:      0000
.tag:    0001
._:      A5A5A5A5
",
    )
    .unwrap();
}
