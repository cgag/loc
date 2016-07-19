extern crate count;

use count::{Count, count_regex, count_manual_bytes_try1, count_manual_bytes_with_iterator};

#[test]
fn test_count_c() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_regex("tests/data/plasma.c"));
}

#[test]
fn test_count_c_manual_bytes() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_manual_bytes_try1("tests/data/plasma.c"));
}


#[test]
fn test_count_c_manual_bytes_with_iterator() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_manual_bytes_with_iterator("tests/data/plasma.c"));
}
