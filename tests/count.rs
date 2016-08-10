extern crate count;

use count::*;

const PLASMA: &'static str = "tests/data/plasma.c";
const PLASMA_EXPECTED: Count = Count {
    code: 32032,
    blank: 8848,
    comment: 3792,
    lines: 44672,
};


#[test]
fn test_plasma_count() {
    assert_eq!(PLASMA_EXPECTED, count(PLASMA));
}

#[test]
fn test_plasma_count_code() {
    assert_eq!(PLASMA_EXPECTED.code, count(PLASMA).code);
}

#[test]
fn test_plasma_count_comment() {
    assert_eq!(PLASMA_EXPECTED.comment, count(PLASMA).comment);
}

#[test]
fn test_plasma_count_blank() {
    assert_eq!(PLASMA_EXPECTED.blank, count(PLASMA).blank);
}

#[test]
fn test_plasma_count_lines() {
    assert_eq!(PLASMA_EXPECTED.lines, count(PLASMA).lines);
}

const FE: &'static str = "tests/data/fe25519.c";
const FE_EXPECTED: Count = Count {
    code: 278,
    blank: 51,
    comment: 8,
    lines: 278 + 51 + 8,
};


#[test]
fn test_fe_count() {
    assert_eq!(FE_EXPECTED, count(FE));
}

#[test]
fn test_fe_code() {
    assert_eq!(FE_EXPECTED.code, count(FE).code);
}

#[test]
fn test_fe_comment() {
    assert_eq!(FE_EXPECTED.comment, count(FE).comment);
}

#[test]
fn test_fe_blank() {
    assert_eq!(FE_EXPECTED.blank, count(FE).blank);
}

#[test]
fn test_fe_lines() {
    assert_eq!(FE_EXPECTED.lines, count(FE).lines);
}

const EBC: &'static str = "tests/data/ebcdic.c";
const EBC_EXPECTED: Count = Count {
    code: 165,
    blank: 18,
    comment: 101,
    lines: 165 + 18 + 101,
};

#[test]
fn test_ebc_count() {
    assert_eq!(EBC_EXPECTED, count(EBC));
}

#[test]
fn test_ebc_code() {
    assert_eq!(EBC_EXPECTED.code, count(EBC).code);
}

#[test]
fn test_ebc_comment() {
    assert_eq!(EBC_EXPECTED.comment, count(EBC).comment);
}

#[test]
fn test_ebc_blank() {
    assert_eq!(EBC_EXPECTED.blank, count(EBC).blank);
}

#[test]
fn test_ebc_lines() {
    assert_eq!(EBC_EXPECTED.lines, count(EBC).lines);
}
