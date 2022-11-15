use crate::*;

#[test]
fn changed_display() {
    let expected = [0x00, 0x00, 0x00, 0x3F, 0xEB, 0x50];
    let actual = [0x00, 0x00, 0x00, 0x3F, 0xFE, 0xED];

    let region = Region {
        label: "sofh.sig".to_string(),
        start: 4,
        end: 6,
    };

    let change = Discrepancy::Changed {
        expect: region.subsection(&expected),
        actual: region.subsection(&actual),
        region: &region,
    };

    assert_eq!(
        change.to_string().as_str(),
        "region \"sofh.sig\" expected 0xEB50 but got 0xFEED"
    );
}

#[test]
fn missing_display() {
    let region = Region {
        label: "sofh.sig".to_string(),
        start: 4,
        end: 6,
    };

    let missing = Discrepancy::MissingRegion { region: &region };

    assert_eq!(
        missing.to_string().as_str(),
        "region \"sofh.sig\" missing, expected a buffer with at least 6 bytes"
    );
}
