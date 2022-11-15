use crate::*;

impl LoadError {
    #[track_caller]
    fn unwrap_parse(self) -> Contextualized<ParseError> {
        match self {
            LoadError::Parse(e) => e,
            e => panic!("unrelated error: {e}"),
        }
    }
}

#[test]
fn duplicate_abs_labels() {
    let err = HexSpec::parse(
        r"
; Simple Open Framing Header
sofh:
.length:     00000054 ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
._:         1234 ; Some Reserved bytes
.version:   0000 ; SBE Version

; Duplicate
sofh:
.length:     00000054 ; SOFH Header
.encoding:   EB50     ; SOFH Encoding
",
    );

    let err = err.unwrap_err();
    let err = err.unwrap_parse();

    assert_eq!(*err, ParseError::DuplicateLabel);
}

#[test]
fn duplicate_rel_labels() {
    let err = HexSpec::parse(
        r"
; Simple Open Framing Header
sofh:
.length:     00000054 ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
._:         1234 ; Some Reserved bytes
.version:   0000 ; SBE Version
.length:     00000054 ; SOFH Header
.encoding:   EB50     ; SOFH Encoding
",
    );

    let err = err.unwrap_err();
    let err = err.unwrap_parse();

    assert_eq!(*err, ParseError::DuplicateLabel);
}

#[test]
fn missing_nibble() {
    let err = HexSpec::parse(
        r"
; Simple Open Framing Header
sofh:
.length:     00 00 00 5  ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
.version:   0000 ; SBE Version
",
    );

    let err = err.unwrap_err();
    let err = err.unwrap_parse();

    assert_eq!(*err, ParseError::MissingNibble);

    assert_eq!(err.context.line, ".length:     00 00 00 5  ; SOFH Header");
    assert_eq!(err.context.lineno, 4);
    assert_eq!(err.context.column, 24);
    assert_eq!(err.context.filename, None);

    //assert!(matches!(err, ParseError::MissingNibble(_)));
}

#[test]
fn error_message() {
    let err = HexSpec::parse(
        r"
; Simple Open Framing Header
sofh:
.length:     00 00 00 5  ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
.version:   0000 ; SBE Version
",
    );

    let err = err.unwrap_err();
    let mut err = err.unwrap_parse();

    err.context.filename = Some("sbe.hexspec".to_string());

    assert_eq!(*err, ParseError::MissingNibble);

    assert_eq!(
        err.to_string(),
        "sbe.hexspec:4:.length:     00 00 00 5  ; SOFH Header\n                                     ^ missing nibble\n"
    );
}

#[test]
fn begin_unachored() {
    let err = HexSpec::parse(
        r"
.absured:
; Simple Open Framing Header
sofh:
.length:     00000054  ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
",
    );

    let err = err.unwrap_err();
    let err = err.unwrap_parse();

    assert_eq!(*err, ParseError::UnanchoredRelativeLabel);
}

#[test]
fn cleared_unanchored() {
    let err = HexSpec::parse(
        r"
; Simple Open Framing Header
sofh:
.length:     00000054  ; SOFH Header
.encoding:   EB50     ; SOFH Encoding

_:
.absured: 0000 0000

; Simple Binary Encoding Header
sbe:
.length:    2A00 ; SBE Block Length
.template:  6200 ; SBE Template ID
.schema:    5B00 ; SBE Schema ID
",
    );

    let err = err.unwrap_err();
    let err = err.unwrap_parse();

    assert_eq!(*err, ParseError::UnanchoredRelativeLabel);
}
