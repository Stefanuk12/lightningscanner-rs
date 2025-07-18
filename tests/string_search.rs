use lightningscanner::{pattern::Pattern, ScanMode, Scanner};

const PATTERN: &str = "S^me??T&stD/ta;?";

const DATA_SET: [u8; 144] = [
    0xdb, 0x2f, 0x16, 0x37, 0xd5, 0xff, 0x12, 0x74, 0x7c, 0xf2, 0x27, 0xed, 0x7b, 0x2e, 0x54, 0x9a,
    0xe2, 0xec, 0x73, 0x9e, 0xbb, 0xd1, 0x42, 0xc2, 0x0c, 0x9e, 0xa3, 0xa1, 0x10, 0xb3, 0x97, 0xf2,
    0xaf, 0x47, 0x43, 0x9f, 0xa0, 0x9e, 0x87, 0x00, 0x76, 0x5c, 0x3a, 0xae, 0x40, 0x30, 0x7f, 0xc0,
    0x53, 0xf4, 0xeb, 0xcc, 0xf2, 0x04, 0x6d, 0x35, 0x5c, 0x88, 0xc3, 0x83, 0xdf, 0x9b, 0xc9, 0x44,
    0x42, 0xcd, 0xe7, 0xf8, 0x21, 0x5b, 0xd6, 0xb8, 0xd1, 0xbe, 0x12, 0x0e, 0x85, 0x34, 0xc4, 0xf9,
    0x03, 0x7e, 0xbc, 0x7b, 0xb9, 0x29, 0xb6, 0x07, 0x31, 0x7e, 0x69, 0xdd, 0x3e, 0x0a, 0xe7, 0x71,
    0xf3, 0xb7, 0x76, 0x3f, 0x36, 0xe1, 0xf3, 0x3b, 0xc6, 0xe5, 0x69, 0xf8, 0x97, 0x67, 0x86, 0x60,
    0x4d, 0x2b, 0xf6, 0x2f, 0x9e, 0x03, 0x5f, 0x56, 0x02, 0x2e, 0x5f, 0x58, 0x9c, 0x6d, 0xa3, 0xf5,
    0x53, 0x5e, 0x6d, 0x65, 0x3f, 0x3f, 0x54, 0x26, 0x73, 0x74, 0x44, 0x2f, 0x74, 0x61, 0x3b, 0x3f,
];

const EXPECTED_FIND: usize = 0x80;

#[test]
#[cfg(target_feature = "avx2")]
fn avx2() {
    let scanner = Scanner::new(PATTERN);
    // SAFETY: DATA_SET is a valid slice
    let result = unsafe { scanner.find(Some(ScanMode::Avx2), DATA_SET.as_ptr(), DATA_SET.len()) };

    let data_set_addr = DATA_SET.as_ptr() as usize;
    let ptr = result.get_addr() as usize;

    assert_eq!(ptr - data_set_addr, EXPECTED_FIND);
}

#[test]
#[cfg(target_feature = "sse4.2")]
fn sse42() {
    let scanner = Scanner::new(PATTERN);
    // SAFETY: DATA_SET is a valid slice
    let result = unsafe { scanner.find(Some(ScanMode::Sse42), DATA_SET.as_ptr(), DATA_SET.len()) };

    let data_set_addr = DATA_SET.as_ptr() as usize;
    let ptr = result.get_addr() as usize;

    assert_eq!(ptr - data_set_addr, EXPECTED_FIND);
}

#[test]
fn scalar() {
    let pattern = Pattern::new_string(PATTERN);
    let scanner = Scanner::from(pattern);

    // SAFETY: DATA_SET is a valid slice
    let result = unsafe { scanner.find(Some(ScanMode::Scalar), DATA_SET.as_ptr(), DATA_SET.len()) };

    println!("{:#p}", result.get_addr());
    let data_set_addr = DATA_SET.as_ptr() as usize;
    let ptr = result.get_addr() as usize;

    assert_eq!(ptr - data_set_addr, EXPECTED_FIND);
}
