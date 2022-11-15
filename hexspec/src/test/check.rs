use std::path::PathBuf;

use crate::*;

#[test]
fn spec_should_pass() {
    let spec = HexSpec::load(PathBuf::from("specs/example.hexspec")).unwrap();

    let should_pass = spec.buf.clone();

    assert!(spec.discrepancies(&should_pass).is_empty());

    spec.validate(&should_pass).unwrap();
}

#[test]
#[should_panic]
fn spec_fail_validate() {
    let spec = HexSpec::load(PathBuf::from("specs/example.hexspec")).unwrap();

    let mut should_fail = spec.buf.clone();

    should_fail[4] = 0xFE;
    should_fail[5] = 0xED;

    spec.validate(&should_fail).unwrap();
}

#[test]
fn spec_fail_exact_discrepancy() {
    let spec = HexSpec::load(PathBuf::from("specs/example.hexspec")).unwrap();

    let mut should_fail = spec.buf.clone();

    should_fail[4] = 0xFE;
    should_fail[5] = 0xED;

    assert_eq!(
        spec.discrepancies(&should_fail),
        vec![Discrepancy::Changed {
            expect: &[0xEB, 0x50],
            actual: &[0xFE, 0xED],
            region: &Region {
                label: "sofh.sig".to_string(),
                start: 4,
                end: 6,
            }
        }]
    );
}
