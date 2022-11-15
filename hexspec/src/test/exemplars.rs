use crate::*;
use std::path::PathBuf;

#[test]
fn bare() {
    let spec = HexSpec::load(PathBuf::from("specs/bare.hexspec")).unwrap();

    assert_eq!(spec.as_ref(), &[0xAB, 0xCD]);
    assert_eq!(spec.regions().count(), 0);
}

#[test]
fn ignore_space() {
    let spec = HexSpec::load(PathBuf::from("specs/space.hexspec")).unwrap();

    assert_eq!(spec.as_ref(), &[0xAB, 0xCD, 0x01, 0xCD]);
    assert_eq!(spec.regions().count(), 0);
}

#[test]
fn comment() {
    let spec = HexSpec::load(PathBuf::from("specs/comment.hexspec")).unwrap();

    assert_eq!(spec.as_ref(), &[0x34, 0x45, 0x43, 0x12]);
    assert_eq!(spec.regions().count(), 0);
}

#[test]
fn abs_labels() {
    let spec = HexSpec::load(PathBuf::from("specs/abs_labels.hexspec")).unwrap();

    assert_eq!(
        spec.as_ref(),
        &[0x00, 0x00, 0x00, 0x54, 0xEB, 0x50, 0x02A, 0x00, 0x62, 0x00, 0x5B, 0x00, 0x00, 0x00]
    );

    let regions: Vec<&Region> = spec.regions().collect();

    assert_eq!(
        regions,
        vec![
            &Region {
                label: "sofh".to_string(),
                start: 0,
                end: 6,
            },
            &Region {
                label: "sbe".to_string(),
                start: 6,
                end: 14,
            }
        ]
    );
}

#[test]
fn abs_unset() {
    let spec = HexSpec::load(PathBuf::from("specs/abs_unset.hexspec")).unwrap();

    assert_eq!(
        spec.as_ref(),
        &[
            0x00, 0x00, 0x00, 0x54, 0xEB, 0x50, 0x00, 0x00, 0x00, 0x00, 0x02A, 0x00, 0x62, 0x00,
            0x5B, 0x00, 0x00, 0x00
        ]
    );

    let regions: Vec<&Region> = spec.regions().collect();

    assert_eq!(
        regions,
        vec![
            &Region {
                label: "sofh".to_string(),
                start: 0,
                end: 6,
            },
            &Region {
                label: "sbe".to_string(),
                start: 10,
                end: 18,
            }
        ]
    );
}

#[test]
fn rel_labels() {
    let spec = HexSpec::load(PathBuf::from("specs/rel_labels.hexspec")).unwrap();

    assert_eq!(
        spec.as_ref(),
        &[0x00, 0x00, 0x00, 0x54, 0xEB, 0x50, 0x02A, 0x00, 0x62, 0x00, 0x5B, 0x00, 0x00, 0x00]
    );

    let regions: Vec<&Region> = spec.regions().collect();

    assert_eq!(
        regions,
        vec![
            &Region {
                label: "sofh.length".to_string(),
                start: 0,
                end: 4,
            },
            &Region {
                label: "sofh.encoding".to_string(),
                start: 4,
                end: 6,
            },
            &Region {
                label: "sofh".to_string(),
                start: 0,
                end: 6,
            },
            &Region {
                label: "sbe.length".to_string(),
                start: 6,
                end: 8,
            },
            &Region {
                label: "sbe.template".to_string(),
                start: 8,
                end: 10,
            },
            &Region {
                label: "sbe.schema".to_string(),
                start: 10,
                end: 12,
            },
            &Region {
                label: "sbe.version".to_string(),
                start: 12,
                end: 14,
            },
            &Region {
                label: "sbe".to_string(),
                start: 6,
                end: 14,
            }
        ]
    );
}

#[test]
fn rel_unset() {
    let spec = HexSpec::load(PathBuf::from("specs/rel_unset.hexspec")).unwrap();

    assert_eq!(
        spec.as_ref(),
        &[
            0x00, 0x00, 0x00, 0x54, 0xEB, 0x50, 0x02A, 0x00, 0x62, 0x00, 0x5B, 0x00, 0x12, 0x34,
            0x00, 0x00
        ]
    );

    let regions: Vec<&Region> = spec.regions().collect();

    assert_eq!(
        regions,
        vec![
            &Region {
                label: "sofh.length".to_string(),
                start: 0,
                end: 4,
            },
            &Region {
                label: "sofh.encoding".to_string(),
                start: 4,
                end: 6,
            },
            &Region {
                label: "sofh".to_string(),
                start: 0,
                end: 6,
            },
            &Region {
                label: "sbe.length".to_string(),
                start: 6,
                end: 8,
            },
            &Region {
                label: "sbe.template".to_string(),
                start: 8,
                end: 10,
            },
            &Region {
                label: "sbe.schema".to_string(),
                start: 10,
                end: 12,
            },
            &Region {
                label: "sbe.version".to_string(),
                start: 14,
                end: 16,
            },
            &Region {
                label: "sbe".to_string(),
                start: 6,
                end: 16,
            }
        ]
    );
}

#[test]
fn label_space() {
    let spec = HexSpec::load(PathBuf::from("specs/label_space.hexspec")).unwrap();

    assert_eq!(spec.as_ref(), &[0xAB, 0xCD, 0x0EF]);

    let regions: Vec<&Region> = spec.regions().collect();

    assert_eq!(
        regions,
        vec![
            &Region {
                label: "spec.foobar".to_string(),
                start: 1,
                end: 2,
            },
            &Region {
                label: "spec.xyz".to_string(),
                start: 2,
                end: 3,
            },
            &Region {
                label: "spec".to_string(),
                start: 0,
                end: 3,
            },
        ]
    );
}

#[test]
fn types_spec() {
    let spec = HexSpec::load("specs/types.hexspec").unwrap();
    let same = HexSpec::load("specs/example.hexspec").unwrap();

    spec.validate(&same).unwrap();
    //assert_eq!(spec.as_ref(), same.as_ref(), "byte buffer is the same");
}
