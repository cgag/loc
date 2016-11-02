extern crate loc;

use loc::*;

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
    println!("Expected {} comment lines, got {}",
             EBC_EXPECTED.comment,
             count(EBC).comment);
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

const DUMB: &'static str = "tests/data/dumb.c";
const DUMB_EXPECTED: Count = Count {
    code: 2,
    blank: 0,
    comment: 3,
    lines: 5,
};

#[test]
fn test_dumb_count() {
    assert_eq!(DUMB_EXPECTED, count(DUMB));
}

#[test]
fn test_dumb_code() {
    assert_eq!(DUMB_EXPECTED.code, count(DUMB).code);
}

#[test]
fn test_dumb_comment() {
    assert_eq!(DUMB_EXPECTED.comment, count(DUMB).comment);
}

#[test]
fn test_dumb_blank() {
    assert_eq!(DUMB_EXPECTED.blank, count(DUMB).blank);
}

#[test]
fn test_dumb_lines() {
    assert_eq!(DUMB_EXPECTED.lines, count(DUMB).lines);
}

const IPL: &'static str = "tests/data/ipl_funcs.c";
const IPL_EXPECTED: Count = Count {
    code: 25,
    blank: 6,
    comment: 43,
    lines: 25 + 6 + 43,
};


#[test]
fn test_ipl_count() {
    assert_eq!(IPL_EXPECTED, count(IPL));
}

#[test]
fn test_ipl_code() {
    assert_eq!(IPL_EXPECTED.code, count(IPL).code);
}

#[test]
fn test_ipl_comment() {
    assert_eq!(IPL_EXPECTED.comment, count(IPL).comment);
}

#[test]
fn test_ipl_blank() {
    assert_eq!(IPL_EXPECTED.blank, count(IPL).blank);
}

#[test]
fn test_ipl_lines() {
    assert_eq!(IPL_EXPECTED.lines, count(IPL).lines);
}

// TODO(cgag): find or make a better testing tool? Or add some simple macros?
const LUA: &'static str = "tests/data/lua.lua";
const LUA_EXPECTED: Count = Count {
    code: 7,
    blank: 1,
    comment: 8,
    lines: 7 + 8 + 1,
};

#[test]
fn test_lua_count() {
    assert_eq!(LUA_EXPECTED, count(LUA));
}

#[test]
fn test_lua_code() {
    assert_eq!(LUA_EXPECTED.code, count(LUA).code);
}

#[test]
fn test_lua_comment() {
    assert_eq!(LUA_EXPECTED.comment, count(LUA).comment);
}

#[test]
fn test_lua_blank() {
    assert_eq!(LUA_EXPECTED.blank, count(LUA).blank);
}

#[test]
fn test_lua_lines() {
    assert_eq!(LUA_EXPECTED.lines, count(LUA).lines);
}
