extern crate count;

use count::*;

#[test]
fn test_count_mmap_safe() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count("tests/data/plasma.c"));
}
