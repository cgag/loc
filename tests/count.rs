extern crate count;

use count::*;

#[test]
fn test_count_regex() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_regex("tests/data/plasma.c"));
}

#[test]
fn test_count_reader() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_reader("tests/data/plasma.c"));
}


#[test]
fn test_count_reader2() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_reader2("tests/data/plasma.c"));
}

#[test]
fn test_count_mmap_safe() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_mmap_safe("tests/data/plasma.c"));
}

#[test]
fn test_count_mmap_unsafe() {
    let exp = Count {
        code: 32032,
        blank: 8848,
        comment: 3792,
        lines: 44672,
    };
    assert_eq!(exp, count_mmap_unsafe("tests/data/plasma.c"));
}




// #[test]
// fn test_count_c_manual_bytes() {
//     let exp = Count {
//         code: 32032,
//         blank: 8848,
//         comment: 3792,
//         lines: 44672,
//     };
//     assert_eq!(exp, count_manual_bytes_try1("tests/data/plasma.c"));
// }


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
